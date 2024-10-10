use std::future::{self, Future};
use std::pin::Pin;

use async_std::task::sleep;
use http_types::{Request, StatusCode};
use serde::de::DeserializeOwned;

use crate::client::request_strategy::{Outcome, RequestStrategy};
use crate::error::{ErrorResponse, StripeError};

pub type Response<T> = Pin<Box<dyn Future<Output = Result<T, StripeError>> + Send>>;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn ok<T: Send + 'static>(ok: T) -> Response<T> {
    Box::pin(future::ready(Ok(ok)))
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn err<T: Send + 'static>(err: StripeError) -> Response<T> {
    Box::pin(future::ready(Err(err)))
}

#[derive(Clone)]
pub struct AsyncStdClient {
    client: surf::Client,
}

impl Default for AsyncStdClient {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncStdClient {
    /// Creates a new client pointed to `https://api.stripe.com/`
    pub fn new() -> Self {
        Self { client: surf::Client::new() }
    }

    pub fn execute<T: DeserializeOwned + Send + 'static>(
        &self,
        request: Request,
        strategy: &RequestStrategy,
    ) -> Response<T> {
        // need to clone here since client could be used across threads.
        // N.B. Client is send sync; cloned clients share the same pool.
        let client = self.client.clone();
        let strategy = strategy.clone();

        Box::pin(async move {
            let bytes = send_inner(&client, request, &strategy).await?;
            let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
            serde_path_to_error::deserialize(json_deserializer).map_err(StripeError::from)
        })
    }
}

async fn send_inner(
    client: &surf::Client,
    mut request: Request,
    strategy: &RequestStrategy,
) -> Result<Vec<u8>, StripeError> {
    let mut tries = 0;
    let mut last_status: Option<StatusCode> = None;
    let mut last_retry_header: Option<bool> = None;

    // if we have no last error, then the strategy is invalid
    let mut last_error = StripeError::ClientError("Invalid strategy".to_string());

    if let Some(key) = strategy.get_key() {
        request.insert_header("Idempotency-Key", key);
    }

    let body = request.body_bytes().await?;

    loop {
        return match strategy.test(last_status, last_retry_header, tries) {
            Outcome::Stop => Err(last_error),
            Outcome::Continue(duration) => {
                if let Some(duration) = duration {
                    sleep(duration).await;
                }

                // we need to clone the request before sending it so we can
                // re-use it if we need to retry. ditto for the body
                let mut request = request.clone();
                request.set_body(body.clone());

                let mut response = match client.send(request).await {
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
                    let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
                    last_error = serde_path_to_error::deserialize(json_deserializer)
                        .map(|mut e: ErrorResponse| {
                            e.error.http_status = status.into();
                            StripeError::from(e.error)
                        })
                        .unwrap_or_else(StripeError::from);
                    last_status = Some(status);
                    last_retry_header = retry;

                    continue;
                }

                Ok(bytes)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use http_types::{Request, Url};
    use httpmock::prelude::*;

    use super::AsyncStdClient;
    use crate::{client::request_strategy::RequestStrategy, StripeError};

    #[async_std::test]
    async fn retry() {
        let client = AsyncStdClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let hello_mock = server.mock(|when, then| {
            when.method(GET).path("/server-errors");
            then.status(500);
        });

        let req = Request::get(Url::parse(&server.url("/server-errors")).unwrap());
        let res = client.execute::<()>(req, &RequestStrategy::Retry(5)).await;

        hello_mock.assert_hits_async(5).await;
        assert!(res.is_err());
    }

    #[async_std::test]
    async fn user_error() {
        let client = AsyncStdClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/v1/missing");
            then.status(404).body("{
                \"error\": {
                  \"message\": \"Unrecognized request URL (GET: /v1/missing). Please see https://stripe.com/docs or we can help at https://support.stripe.com/.\",
                  \"type\": \"invalid_request_error\"
                }
              }
              ");
        });

        let req = Request::get(Url::parse(&server.url("/v1/missing")).unwrap());
        let res = client.execute::<()>(req, &RequestStrategy::Retry(3)).await;

        mock.assert_hits_async(1).await;

        match res {
            Err(StripeError::Stripe(x)) => println!("{:?}", x),
            _ => panic!("Expected stripe error {:?}", res),
        }
    }

    #[async_std::test]
    async fn retry_header() {
        let client = AsyncStdClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let hello_mock = server.mock(|when, then| {
            when.method(GET).path("/server-errors");
            then.status(500).header("Stripe-Should-Retry", "false");
        });

        let req = Request::get(Url::parse(&server.url("/server-errors")).unwrap());
        let res = client.execute::<()>(req, &RequestStrategy::Retry(5)).await;

        hello_mock.assert_hits_async(1).await;
        assert!(res.is_err());
    }

    #[async_std::test]
    async fn retry_body() {
        let client = AsyncStdClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let hello_mock = server.mock(|when, then| {
            when.method(POST).path("/server-errors").body("body");
            then.status(500);
        });

        let mut req = Request::post(Url::parse(&server.url("/server-errors")).unwrap());
        req.set_body("body");
        let res = client.execute::<()>(req, &RequestStrategy::Retry(5)).await;

        hello_mock.assert_hits_async(5).await;
        assert!(res.is_err());
    }
}
