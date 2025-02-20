use std::fmt::Write as _;

use hyper::body::Bytes;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use hyper::http::request::Builder;
use hyper::http::{HeaderName, HeaderValue};
use hyper::{Body, Request, StatusCode};
use miniserde::json::from_str;
use stripe_client_core::{CustomizedStripeRequest, RequestBuilder, StripeMethod};
use stripe_client_core::{Outcome, RequestStrategy};
use stripe_shared::AccountId;

use crate::hyper::client_builder::{ClientBuilder, ClientConfig};
use crate::StripeError;

/// A client for making Stripe API requests.
#[derive(Clone, Debug)]
pub struct Client {
    client: hyper::Client<crate::hyper::connector::Connector, Body>,
    config: ClientConfig,
}

// TODO: this looks to be much simpler in hyper 1.x
// There's probably also a better way to do this now...
fn clone_builder(builder: &Builder) -> Builder {
    let mut req_builder =
        Request::builder().uri(builder.uri_ref().unwrap()).method(builder.method_ref().unwrap());
    for (k, v) in builder.headers_ref().unwrap() {
        req_builder = req_builder.header(k, v);
    }
    req_builder
}

impl Client {
    pub(crate) fn from_config(config: ClientConfig) -> Self {
        Self {
            client: hyper::Client::builder()
                .pool_max_idle_per_host(0)
                .build(crate::hyper::connector::create()),
            config,
        }
    }

    /// Construct a `client` with the given secret key and a default configuration.
    ///
    /// # Panics
    /// This method panics if secret key is not usable as a header value.
    pub fn new(secret_key: impl Into<String>) -> Self {
        ClientBuilder::new(secret_key).build().expect("invalid secret provided")
    }

    fn get_account_id_header(
        &self,
        account_id_override: Option<AccountId>,
    ) -> Result<Option<HeaderValue>, StripeError> {
        if let Some(overridden) = account_id_override {
            return Ok(Some(HeaderValue::from_str(overridden.as_str()).map_err(|_| {
                StripeError::ConfigError("invalid account id set in customizations".into())
            })?));
        }
        Ok(self.config.account_id.clone())
    }

    fn construct_request(
        &self,
        req: RequestBuilder,
        account_id: Option<AccountId>,
    ) -> Result<(Builder, Option<Bytes>), StripeError> {
        let mut uri = format!("{}v1{}", self.config.api_base, req.path);
        if let Some(query) = req.query {
            let _ = write!(uri, "?{query}");
        }

        let mut builder = Request::builder()
            .method(conv_stripe_method(req.method))
            .uri(uri)
            .header(AUTHORIZATION, self.config.secret.clone())
            .header(USER_AGENT, self.config.user_agent.clone())
            .header(HeaderName::from_static("stripe-version"), self.config.stripe_version.clone());

        if let Some(client_id) = &self.config.client_id {
            builder = builder.header(HeaderName::from_static("client-id"), client_id.clone());
        }
        if let Some(account_id) = self.get_account_id_header(account_id)? {
            builder = builder.header(HeaderName::from_static("stripe-account"), account_id);
        }

        let body = if let Some(body) = req.body {
            builder = builder.header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
            Some(Bytes::from(body))
        } else {
            None
        };
        Ok((builder, body))
    }

    async fn send_inner(
        &self,
        body: Option<Bytes>,
        mut req_builder: Builder,
        strategy: RequestStrategy,
    ) -> Result<Bytes, StripeError> {
        let mut tries = 0;
        let mut last_status: Option<StatusCode> = None;
        let mut last_retry_header: Option<bool> = None;
        let mut last_error = StripeError::ClientError("invalid strategy".into());

        if let Some(key) = strategy.get_key() {
            const HEADER_NAME: HeaderName = HeaderName::from_static("idempotency-key");
            req_builder = req_builder.header(HEADER_NAME, key.as_str());
        }

        loop {
            return match strategy.test(last_status.map(|s| s.as_u16()), last_retry_header, tries) {
                Outcome::Stop => Err(last_error),
                Outcome::Continue(duration) => {
                    if let Some(duration) = duration {
                        tokio::time::sleep(duration).await;
                    }

                    let req_body = if let Some(body) = body.clone() {
                        Body::from(body)
                    } else {
                        Body::empty()
                    };
                    let req = clone_builder(&req_builder).body(req_body)?;
                    let response = match self.client.request(req).await {
                        Ok(resp) => resp,
                        Err(err) => {
                            last_error = StripeError::from(err);
                            tries += 1;
                            continue;
                        }
                    };
                    let status = response.status();
                    let retry = response
                        .headers()
                        .get("Stripe-Should-Retry")
                        .and_then(|s| s.to_str().ok())
                        .and_then(|s| s.parse().ok());

                    let bytes = hyper::body::to_bytes(response.into_body()).await?;
                    if !status.is_success() {
                        tries += 1;

                        let str = std::str::from_utf8(bytes.as_ref()).map_err(|_| {
                            StripeError::JSONDeserialize("Response was not valid UTF-8".into())
                        })?;
                        last_error = from_str(str)
                            .map(|e: stripe_shared::Error| {
                                StripeError::Stripe(e.error, status.as_u16())
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

fn conv_stripe_method(method: StripeMethod) -> hyper::Method {
    match method {
        StripeMethod::Get => hyper::Method::GET,
        StripeMethod::Post => hyper::Method::POST,
        StripeMethod::Delete => hyper::Method::DELETE,
    }
}

impl stripe_client_core::StripeClient for Client {
    type Err = StripeError;

    async fn execute(&self, req_full: CustomizedStripeRequest) -> Result<Bytes, Self::Err> {
        let (req, config) = req_full.into_pieces();
        let (builder, body) = self.construct_request(req, config.account_id)?;

        let request_strategy =
            config.request_strategy.unwrap_or_else(|| self.config.request_strategy.clone());
        self.send_inner(body, builder, request_strategy).await
    }
}
