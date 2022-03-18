use std::convert::TryInto;
use std::future::{self, Future};
use std::pin::Pin;

use http_types::{Request, StatusCode};
use hyper::http;
use hyper::{
    client::HttpConnector,
    header::{HeaderMap, HeaderName, HeaderValue},
    Body,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio::time::sleep;

use crate::client::request_strategy::{Outcome, RequestStrategy};
use crate::error::{ErrorResponse, StripeError};
use crate::params::{AppInfo, Headers};
use crate::resources::ApiVersion;

#[cfg(feature = "hyper-rustls")]
mod connector {
    use hyper::client::{connect::dns::GaiResolver, HttpConnector};
    pub use hyper_rustls::HttpsConnector;

    pub fn create() -> HttpsConnector<HttpConnector<GaiResolver>> {
        HttpsConnector::with_native_roots()
    }
}

#[cfg(feature = "hyper-tls")]
mod connector {
    use hyper::client::{connect::dns::GaiResolver, HttpConnector};
    pub use hyper_tls::HttpsConnector;

    pub fn create() -> HttpsConnector<HttpConnector<GaiResolver>> {
        HttpsConnector::new()
    }
}

#[cfg(all(feature = "hyper-tls", feature = "hyper-rustls"))]
compile_error!("You must enable only one TLS implementation");

type HttpClient = hyper::Client<connector::HttpsConnector<HttpConnector>, Body>;

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
pub struct TokioClient {
    client: HttpClient,
}

impl TokioClient {
    pub fn new() -> Self {
        Self { client: hyper::Client::builder().build(connector::create()) }
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
    client: &HttpClient,
    mut request: Request,
    strategy: &RequestStrategy,
) -> Result<hyper::body::Bytes, StripeError> {
    let mut tries = 0;
    let mut last_status: Option<StatusCode> = None;
    let mut last_retry_header: Option<bool> = None;

    if let Some(key) = strategy.get_key() {
        request.insert_header("Idempotency-Key", key);
    }

    loop {
        return match strategy.test(last_status, last_retry_header, tries) {
            Outcome::Stop => Err(StripeError::Timeout),
            Outcome::Continue(duration) => {
                if let Some(duration) = duration {
                    sleep(duration).await;
                }

                // note: http::Request provides no easy way to clone, so we perform
                //       the conversion from the clonable http_types::Request each time
                //       obviously cloning before the first request is not ideal
                let response = client.request(convert_request(request.clone()).await).await?;
                let status = response.status();
                let retry = response
                    .headers()
                    .get("Stripe-Should-Retry")
                    .and_then(|s| s.to_str().unwrap().parse().ok());

                if !status.is_success() {
                    tries += 1;
                    last_status = Some(status.into());
                    last_retry_header = retry;
                    println!("failed! {} {:?} {:?}", tries, last_status, request.clone());
                    continue;
                }

                Ok(hyper::body::to_bytes(response.into_body()).await?)
            }
        };
    }
}

/// convert an http_types::Request with a http_types::Body into a http::Request<hyper::Body>
///
/// note: this is necesarry because `http` deliberately does not support a `Body` type
///       so hyper has a `Body` for which http_types cannot provide automatic conversion.
async fn convert_request(request: http_types::Request) -> http::Request<hyper::Body> {
    let request: http::Request<_> = request.into();
    let (a, b) = request.into_parts();
    let body = match b.into_bytes().await {
        Ok(body) => hyper::Body::from(body),
        Err(e) => hyper::Body::empty(),
    };
    http::Request::from_parts(a, body)
}

#[cfg(test)]
mod tests {
    use http_types::{Method, Request};
    use hyper::{body::to_bytes, Body, Request as HyperRequest};

    use super::convert_request;

    const TEST_URL: &str = "https://api.stripe.com/v1/";

    #[tokio::test]
    async fn basic_conversion() {
        req_equal(
            convert_request(Request::new(Method::Get, TEST_URL)).await,
            HyperRequest::builder()
                .method("GET")
                .uri("http://test.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn bytes_body_conversion() {
        let body = "test".as_bytes();

        let mut req = Request::new(Method::Post, TEST_URL);
        req.set_body(body);

        req_equal(
            convert_request(req).await,
            HyperRequest::builder().method("POST").uri(TEST_URL).body(Body::from(body)).unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn string_body_conversion() {
        let body = "test";

        let mut req = Request::new(Method::Post, TEST_URL);
        req.set_body(body);

        req_equal(
            convert_request(req).await,
            HyperRequest::builder().method("POST").uri(TEST_URL).body(Body::from(body)).unwrap(),
        )
        .await;
    }

    async fn req_equal(a: HyperRequest<Body>, b: HyperRequest<Body>) {
        let (a_parts, a_body) = a.into_parts();
        let (b_parts, b_body) = b.into_parts();

        assert_eq!(a_parts.method, b_parts.method);
        assert_eq!(to_bytes(a_body).await.unwrap().len(), to_bytes(b_body).await.unwrap().len());
    }
}
