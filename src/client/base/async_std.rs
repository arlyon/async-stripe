#![warn(clippy::unwrap_used)]

use std::future::{self, Future};
use std::pin::Pin;

use http_types::{Request, StatusCode};
use serde::de::DeserializeOwned;
use surf::{Body, Url};

use crate::client::request_strategy::{Outcome, RequestStrategy};
use crate::error::{ErrorResponse, StripeError};
use crate::params::{AppInfo, Headers};
use crate::resources::ApiVersion;

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
            serde_json::from_slice(&bytes).map_err(StripeError::from)
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

    if let Some(key) = strategy.get_key() {
        request.insert_header("Idempotency-Key", key);
    }

    loop {
        match strategy.test(last_status, last_retry_header, tries) {
            Outcome::Stop => Err(StripeError::Timeout),
            Outcome::Continue(sleep) => {
                if let Some(sleep) = sleep {
                    std::thread::sleep(sleep);
                }

                let mut res = client.send(request.clone()).await.unwrap();
                let status = res.status();
                let retry =
                    res.header("Stripe-Should-Retry").and_then(|s| s.to_string().parse().ok());

                if !status.is_success() {
                    tries += 1;
                    last_status = Some(status);
                    last_retry_header = retry;
                    continue;
                }

                res.body_bytes().await.map_err(StripeError::from)
            }
        };
    }
}
