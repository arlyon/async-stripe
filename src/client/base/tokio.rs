use std::future::{self, Future};
use std::pin::Pin;

use http_types::{Request, StatusCode};
use hyper::http;
use hyper::{client::HttpConnector, Body};
use serde::de::DeserializeOwned;
use tokio::time::sleep;

use crate::client::request_strategy::{Outcome, RequestStrategy};
use crate::error::{ErrorResponse, StripeError};

#[cfg(feature = "hyper-rustls-native")]
mod connector {
    use hyper::client::{connect::dns::GaiResolver, HttpConnector};
    pub use hyper_rustls::HttpsConnector;
    use hyper_rustls::HttpsConnectorBuilder;

    pub fn create() -> HttpsConnector<HttpConnector<GaiResolver>> {
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build()
    }
}

#[cfg(feature = "hyper-rustls-webpki")]
mod connector {
    use hyper::client::{connect::dns::GaiResolver, HttpConnector};
    pub use hyper_rustls::HttpsConnector;
    use hyper_rustls::HttpsConnectorBuilder;

    pub fn create() -> HttpsConnector<HttpConnector<GaiResolver>> {
        HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build()
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

impl Default for TokioClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TokioClient {
    pub fn new() -> Self {
        Self {
            client: hyper::Client::builder().pool_max_idle_per_host(0).build(connector::create()),
        }
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
    client: &HttpClient,
    mut request: Request,
    strategy: &RequestStrategy,
) -> Result<hyper::body::Bytes, StripeError> {
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

                // note: http::Request provides no easy way to clone, so we perform
                //       the conversion from the clonable http_types::Request each time
                //       obviously cloning before the first request is not ideal
                let mut request = request.clone();
                request.set_body(body.clone());

                let response = match client.request(convert_request(request).await).await {
                    Ok(response) => response,
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
                    let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
                    last_error = serde_path_to_error::deserialize(json_deserializer)
                        .map(|mut e: ErrorResponse| {
                            e.error.http_status = status.into();
                            StripeError::from(e.error)
                        })
                        .unwrap_or_else(StripeError::from);
                    last_status = Some(
                        // NOTE: StatusCode::from can panic here, so fall back to InternalServerError
                        //       see https://github.com/http-rs/http-types/blob/ac5d645ce5294554b86ebd49233d3ec01665d1d7/src/hyperium_http.rs#L20-L24
                        StatusCode::try_from(u16::from(status))
                            .unwrap_or(StatusCode::InternalServerError),
                    );
                    last_retry_header = retry;
                    continue;
                }

                Ok(bytes)
            }
        };
    }
}

/// convert an http_types::Request with a http_types::Body into a http::Request<hyper::Body>
///
/// note: this is necesarry because `http` deliberately does not support a `Body` type
///       so hyper has a `Body` for which http_types cannot provide automatic conversion.
async fn convert_request(mut request: http_types::Request) -> http::Request<hyper::Body> {
    let body = request.body_bytes().await.expect("We know the data is a valid bytes object.");
    let request: http::Request<_> = request.into();
    http::Request::from_parts(request.into_parts().0, hyper::Body::from(body))
}

#[cfg(test)]
mod tests {
    use http_types::{Method, Request, Url};
    use httpmock::prelude::*;
    use hyper::{body::to_bytes, Body, Request as HyperRequest};

    use super::convert_request;
    use super::TokioClient;
    use crate::client::request_strategy::RequestStrategy;
    use crate::StripeError;

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

    #[tokio::test]
    async fn retry() {
        let client = TokioClient::new();

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

    #[tokio::test]
    async fn user_error() {
        let client = TokioClient::new();

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

    #[tokio::test]
    async fn nice_serde_error() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        struct DataType {
            // Allowing dead code since used for deserialization
            #[allow(dead_code)]
            id: String,
            #[allow(dead_code)]
            name: String,
        }

        let client = TokioClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/v1/odd_data");
            then.status(200).body(
                "{
                \"id\": \"test\",
                \"name\": 10
              }
              ",
            );
        });

        let req = Request::get(Url::parse(&server.url("/v1/odd_data")).unwrap());
        let res = client.execute::<DataType>(req, &RequestStrategy::Retry(3)).await;

        mock.assert_hits_async(1).await;

        match res {
            Err(StripeError::JSONSerialize(err)) => {
                println!("Error: {:?} Path: {:?}", err.inner(), err.path().to_string())
            }
            _ => panic!("Expected stripe error {:?}", res),
        }
    }

    #[tokio::test]
    async fn retry_header() {
        let client = TokioClient::new();

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

    #[tokio::test]
    async fn retry_body() {
        let client = TokioClient::new();

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
