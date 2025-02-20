use async_std::task::sleep;
use http_types::{Body, Request, StatusCode};
use miniserde::json::from_str;
use stripe_client_core::{
    CustomizedStripeRequest, Outcome, RequestBuilder, RequestStrategy, StripeMethod,
};
use stripe_shared::AccountId;

use crate::async_std::config::ClientConfig;
use crate::async_std::ClientBuilder;
use crate::StripeError;

/// A client for making Stripe API requests.
#[derive(Clone, Debug)]
pub struct Client {
    client: surf::Client,
    config: ClientConfig,
}

impl Client {
    /// Construct a `client` with the given secret key and a default configuration.
    ///
    /// # Panics
    /// This method panics if secret key is not usable as a header value.
    pub fn new(secret_key: impl Into<String>) -> Self {
        ClientBuilder::new(secret_key).build().expect("invalid secret provided")
    }

    pub(crate) fn from_config(config: ClientConfig) -> Self {
        Self { client: surf::Client::new(), config }
    }

    fn create_request(
        &self,
        req_builder: RequestBuilder,
        account_id: Option<AccountId>,
    ) -> Request {
        let mut url = self.config.api_base.clone();
        url.set_path(&format!("v1/{}", req_builder.path.trim_start_matches('/')));
        if let Some(query) = &req_builder.query {
            url.set_query(Some(query));
        }

        let mut req = Request::new(conv_stripe_method(req_builder.method), url);
        if let Some(body) = req_builder.body {
            req.set_body(Body::from_string(body));
            req.insert_header("content-type", "application/x-www-form-urlencoded");
        }

        req.insert_header("authorization", format!("Bearer {}", self.config.secret));

        for (key, value) in
            self.config.to_headers_array().iter().filter_map(|(k, v)| v.map(|v| (*k, v)))
        {
            req.insert_header(key, value);
        }

        // If an account id is specified, it should override the default configuration
        if let Some(acct_id) = account_id {
            req.insert_header("Stripe-Account", acct_id.as_str());
        }

        req
    }

    async fn send_inner(
        &self,
        mut request: Request,
        strategy: RequestStrategy,
    ) -> Result<Vec<u8>, StripeError> {
        let mut tries = 0;
        let mut last_status: Option<StatusCode> = None;
        let mut last_retry_header: Option<bool> = None;

        // if we have no last error, then the strategy is invalid
        let mut last_error = StripeError::ClientError("Invalid strategy".to_string());

        if let Some(key) = strategy.get_key() {
            request.insert_header("idempotency-key", key.as_str());
        }

        let body = request.body_bytes().await?;

        loop {
            return match strategy.test(last_status.map(|s| s.into()), last_retry_header, tries) {
                Outcome::Stop => Err(last_error),
                Outcome::Continue(duration) => {
                    if let Some(duration) = duration {
                        sleep(duration).await;
                    }

                    // we need to clone the request before sending it so we can
                    // re-use it if we need to retry. ditto for the body
                    let mut request = request.clone();
                    request.set_body(body.clone());

                    let mut response = match self.client.send(request).await {
                        Ok(response) => response,
                        Err(err) => {
                            last_error = StripeError::from(err);
                            tries += 1;
                            continue;
                        }
                    };

                    let status = response.status();
                    let retry = response
                        .header("Stripe-Should-Retry")
                        .and_then(|s| s.last().as_str().parse().ok());

                    // if this fails parsing, we can probably just exit
                    let bytes = response.body_bytes().await?;

                    if !status.is_success() {
                        tries += 1;
                        let str = std::str::from_utf8(bytes.as_ref()).map_err(|_| {
                            StripeError::JSONDeserialize("Response was not valid UTF-8".into())
                        })?;
                        last_error = from_str(str)
                            .map(|e: stripe_shared::Error| {
                                StripeError::Stripe(e.error, status.into())
                            })
                            .unwrap_or_else(|_| {
                                StripeError::JSONDeserialize(
                                    "error deserializing Stripe error".into(),
                                )
                            });
                        last_status = Some(status);
                        last_retry_header = retry;

                        continue;
                    }

                    Ok(bytes)
                }
            };
        }
    }
}

fn conv_stripe_method(method: StripeMethod) -> http_types::Method {
    match method {
        StripeMethod::Get => http_types::Method::Get,
        StripeMethod::Post => http_types::Method::Post,
        StripeMethod::Delete => http_types::Method::Delete,
    }
}

impl stripe_client_core::StripeClient for Client {
    type Err = StripeError;

    async fn execute(&self, req_full: CustomizedStripeRequest) -> Result<bytes::Bytes, Self::Err> {
        let (req, config) = req_full.into_pieces();
        let request_strategy =
            config.request_strategy.unwrap_or_else(|| self.config.request_strategy.clone());
        let req = self.create_request(req, config.account_id);
        let bytes = self.send_inner(req, request_strategy).await?;
        Ok(bytes::Bytes::from(bytes))
    }
}
