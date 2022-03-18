use std::convert::TryInto;
use std::future::{self, Future};
use std::pin::Pin;

use http_types::Request;
use hyper::http;
use hyper::{
    client::HttpConnector,
    header::{HeaderMap, HeaderName, HeaderValue},
    Body,
};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::client::request_strategy::RequestStrategy;
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
    let request = convert_request(request).await;

    let response = client.request(request).await?;
    let status = response.status();
    let bytes = hyper::body::to_bytes(response.into_body()).await?;
    if !status.is_success() {
        Err(serde_json::from_slice(&bytes)
            .map(|mut e: ErrorResponse| {
                e.error.http_status = status.into();
                StripeError::from(e.error)
            })
            .unwrap_or_else(StripeError::from))
    } else {
        Ok(bytes)
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
