use std::{sync::Arc, time::Duration};

use http_types::Request;
use serde::de::DeserializeOwned;

use crate::client::base::tokio::TokioClient;
use crate::client::request_strategy::RequestStrategy;
use crate::error::StripeError;

/// The delay after which the blocking `Client` will assume the request has failed.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

pub type Response<T> = Result<T, StripeError>;

#[inline(always)]
pub(crate) fn ok<T>(ok: T) -> Response<T> {
    Ok(ok)
}

#[inline(always)]
pub(crate) fn err<T>(err: crate::StripeError) -> Response<T> {
    Err(err)
}

#[derive(Clone)]
pub struct TokioBlockingClient {
    inner: TokioClient,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl Default for TokioBlockingClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TokioBlockingClient {
    /// Creates a new client pointed to `https://api.stripe.com/`
    pub fn new() -> TokioBlockingClient {
        TokioBlockingClient::from_async(TokioClient::new())
    }

    fn from_async(inner: TokioClient) -> TokioBlockingClient {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time() // use separate `io/time` instead of `all` to ensure `tokio/time` is enabled
            .build()
            .expect("should be able to get a runtime");
        TokioBlockingClient { inner, runtime: Arc::new(runtime) }
    }

    pub fn execute<T: DeserializeOwned + Send + 'static>(
        &self,
        request: Request,
        strategy: &RequestStrategy,
    ) -> Response<T> {
        let future = self.inner.execute(request, strategy);
        match self.runtime.block_on(async {
            // N.B. The `tokio::time::timeout` must be called from within a running async
            //      context or else it will panic (it registers with the thread-local timer).
            tokio::time::timeout(DEFAULT_TIMEOUT, future).await
        }) {
            Ok(finished) => finished,
            Err(_) => Err(StripeError::Timeout),
        }
    }
}
