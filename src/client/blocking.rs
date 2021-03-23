use std::{cell::RefCell, sync::Arc, time::Duration};

use serde::de::DeserializeOwned;

use crate::client::tokio::Client as AsyncClient;
use crate::error::StripeError;
use crate::params::Headers;

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
pub struct Client {
    inner: AsyncClient,
    runtime: Arc<RefCell<tokio::runtime::Runtime>>,
}

impl Client {
    /// Creates a new client pointed to `https://api.stripe.com/`
    pub fn new(secret_key: impl Into<String>) -> Client {
        Client::from_async(AsyncClient::new(secret_key))
    }

    /// Creates a new client posted to a custom `scheme://host/`
    pub fn from_url(scheme_host: impl Into<String>, secret_key: impl Into<String>) -> Client {
        Client::from_async(AsyncClient::from_url(scheme_host, secret_key))
    }

    fn from_async(inner: AsyncClient) -> Client {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time() // use separate `io/time` instead of `all` to ensure `tokio/time` is enabled
            .build()
            .expect("should be able to get a runtime");
        Client { inner, runtime: Arc::new(RefCell::new(runtime)) }
    }

    /// Clones a new client with different headers.
    ///
    /// This is the recommended way to send requests for many different Stripe accounts
    /// or with different Meta, Extra, and Expand headers while using the same secret key.
    pub fn with_headers(&self, headers: Headers) -> Client {
        Client { inner: self.inner.with_headers(headers), runtime: self.runtime.clone() }
    }

    pub fn set_app_info(&mut self, name: String, version: Option<String>, url: Option<String>) {
        self.inner.set_app_info(name, version, url);
    }

    /// Sets a value for the Stripe-Account header
    ///
    /// This is recommended if you are acting as only one Account for the lifetime of the client.
    /// Otherwise, prefer `client.with(Headers{stripe_account: "acct_ABC", ..})`.
    pub fn set_stripe_account(&mut self, account_id: impl Into<String>) {
        self.inner.set_stripe_account(account_id)
    }

    /// Make a `GET` http request with just a path
    pub fn get<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        self.send_blocking(self.inner.get(path))
    }

    /// Make a `GET` http request with url query parameters
    pub fn get_query<T: DeserializeOwned + Send + 'static, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        self.send_blocking(self.inner.get_query(path, params))
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        self.send_blocking(self.inner.delete(path))
    }

    /// Make a `DELETE` http request with url query parameters
    pub fn delete_query<T: DeserializeOwned + Send + 'static, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        self.send_blocking(self.inner.delete_query(path, params))
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        self.send_blocking(self.inner.post(path))
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned + Send + 'static, F: serde::Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        self.send_blocking(self.inner.post_form(path, form))
    }

    fn send_blocking<T: DeserializeOwned + Send + 'static>(
        &self,
        request: super::tokio::Response<T>,
    ) -> Response<T> {
        match self.runtime.borrow_mut().block_on(async {
            // N.B. The `tokio::time::timeout` must be called from within a running async
            //      context or else it will panic (it registers with the thread-local timer).
            tokio::time::timeout(DEFAULT_TIMEOUT, request).await
        }) {
            Ok(finished) => finished,
            Err(_) => Err(StripeError::Timeout),
        }
    }
}
