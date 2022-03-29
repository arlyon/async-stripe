use std::future::{self, Future};
use std::pin::Pin;

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
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    /// Creates a new client pointed to `https://api.stripe.com/`
    pub fn new() -> Self {
        Self { client: reqwest::Client::new() }
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
            serde_json::from_slice(&bytes).map_err(StripeError::from)
        })
    }
}

async fn send_inner(
    client: &reqwest::Client,
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
                    std::thread::sleep(duration); // todo(arlyon): fix
                }

                // note: http::Request provides no easy way to clone, so we perform
                //       the conversion from the clonable http_types::Request each time
                //       obviously cloning before the first request is not ideal
                let mut request = request.clone();
                request.set_body(body.clone());

                let mut response = match client.execute(convert_request(request).await).await {
                    Ok(response) => response,
                    Err(err) => {
                        last_error = StripeError::from(err);
                        continue;
                    }
                };

                let status = response.status();
                let retry = response
                    .headers().get("Stripe-Should-Retry")
                    .and_then(|s| s.to_str().ok())
                    .and_then(|s| s.parse().ok());

                // if this fails parsing, we can probably just exit
                let bytes = response.bytes().await?;

                if !status.is_success() {
                    tries += 1;
                    last_error = serde_json::from_slice(&bytes)
                        .map(|mut e: ErrorResponse| {
                            e.error.http_status = status.into();
                            StripeError::from(e.error)
                        })
                        .unwrap_or_else(StripeError::from);
                    last_status = Some(http::StatusCode::from(status).into());
                    last_retry_header = retry;

                    continue;
                }

                Ok(bytes.to_vec())
            }
        };
    }
}

/// convert an http_types::Request with a http_types::Body into a http::Request<hyper::Body>
///
/// note: this is necesarry because `http` deliberately does not support a `Body` type
///       so hyper has a `Body` for which http_types cannot provide automatic conversion.
async fn convert_request(mut request: http_types::Request) -> reqwest::Request {
    let body = request.body_bytes().await.expect("We know the data is a valid bytes object.");
    let request: http::Request<_> = request.into();
    http::Request::from_parts(request.into_parts().0, reqwest::Body::from(body)).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use http_types::{Request, Url};
    use httpmock::prelude::*;

    use super::ReqwestClient;
    use crate::{client::request_strategy::RequestStrategy, StripeError};

    #[async_std::test]
    async fn retry() {
        let client = ReqwestClient::new();

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
        let client = ReqwestClient::new();

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
        let client = ReqwestClient::new();

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
        let client = ReqwestClient::new();

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
