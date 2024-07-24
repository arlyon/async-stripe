//! A blocking variant of the Stripe client.
//!
//! The blocking client will block the current thread to execute. Note that
//! this client will panic if used within a async runtime. It is recommended
//! to use the async client within an async context.
//!
//! # Optional
//!
//! This requires enabling the `blocking` feature gate.
use std::{sync::Arc, time::Duration};

use hyper::body::Bytes;
use stripe_client_core::{CustomizedStripeRequest, StripeBlockingClient, StripeClient};

use crate::error::StripeError;

/// The delay after which the blocking `Client` will assume the request has failed.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// A blocking client for making Stripe API requests.
#[derive(Clone, Debug)]
pub struct Client {
    inner: crate::Client,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl Client {
    /// Creates a new client pointed to `https://api.stripe.com/`
    ///
    /// # Panics
    /// This method panics if called from within an async runtime.
    pub fn new(secret_key: impl Into<String>) -> Client {
        Client::from_async(crate::Client::new(secret_key))
    }

    pub(crate) fn from_async(inner: crate::Client) -> Client {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time() // use separate `io/time` instead of `all` to ensure `tokio/time` is enabled
            .build()
            .expect("should be able to get a runtime");
        Client { inner, runtime: Arc::new(runtime) }
    }
}

impl StripeBlockingClient for Client {
    type Err = StripeError;

    fn execute(&self, req: CustomizedStripeRequest) -> Result<Bytes, Self::Err> {
        let future = self.inner.execute(req);
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
