#![warn(clippy::unwrap_used)]

use std::future::{self, Future};
use std::pin::Pin;

use serde::de::DeserializeOwned;
use surf::{Body, Url};

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
pub struct Client {
    host: Url,
    api_root: String,
    client: surf::Client,
    secret_key: String,
    headers: Headers,
    app_info: Option<AppInfo>,
}

impl Client {
    /// Creates a new client pointed to `https://api.stripe.com/`
    pub fn new(secret_key: impl Into<String>) -> Client {
        Client::from_url("https://api.stripe.com/", secret_key)
    }

    /// Creates a new client posted to a custom `scheme://host/`
    pub fn from_url<'a>(scheme_host: impl Into<&'a str>, secret_key: impl Into<String>) -> Client {
        let host = Url::parse(scheme_host.into()).expect("invalid url");
        let client = surf::Client::new();
        let headers =
            Headers { stripe_version: Some(ApiVersion::V2020_08_27), ..Default::default() };

        Client {
            host,
            api_root: "v1".to_string(),
            client,
            secret_key: secret_key.into(),
            headers,
            app_info: Some(AppInfo::default()),
        }
    }

    /// Clones a new client with different headers.
    ///
    /// This is the recommended way to send requests for many different Stripe accounts
    /// or with different Meta, Extra, and Expand headers while using the same secret key.
    pub fn with_headers(&self, headers: Headers) -> Client {
        let mut client = self.clone();
        client.headers = headers;
        client
    }

    pub fn set_app_info(&mut self, name: String, version: Option<String>, url: Option<String>) {
        self.app_info = Some(AppInfo { name, url, version });
    }

    /// Sets a value for the Stripe-Account header
    ///
    /// This is recommended if you are acting as only one Account for the lifetime of the client.
    /// Otherwise, prefer `client.with(Headers{stripe_account: "acct_ABC", ..})`.
    pub fn set_stripe_account<S: Into<String>>(&mut self, account_id: S) {
        self.headers.stripe_account = Some(account_id.into());
    }

    /// Make a `GET` http request with just a path
    pub fn get<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        let req = surf::Request::builder(surf::http::Method::Get, url).build();
        send(&self.client, self.set_headers(req))
    }

    /// Make a `GET` http request with url query parameters
    pub fn get_query<T: DeserializeOwned + Send + 'static, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(err) => return Box::pin(future::ready(Err(err))),
            Ok(ok) => ok,
        };
        let req = surf::Request::builder(surf::http::Method::Get, url).build();
        send(&self.client, self.set_headers(req))
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        let req = surf::Request::builder(surf::http::Method::Delete, url).build();
        send(&self.client, self.set_headers(req))
    }

    /// Make a `DELETE` http request with url query parameters
    pub fn delete_query<T: DeserializeOwned + Send + 'static, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(err) => return Box::pin(future::ready(Err(err))),
            Ok(ok) => ok,
        };
        let req = surf::Request::builder(surf::http::Method::Delete, url).build();
        send(&self.client, self.set_headers(req))
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        let req = surf::Request::builder(surf::http::Method::Post, url).build();
        send(&self.client, self.set_headers(req))
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned + Send + 'static, F: serde::Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        let url = self.url(path);
        let mut req = surf::Request::builder(surf::http::Method::Post, url)
            .body(match serde_qs::to_string(&form) {
                Err(err) => {
                    return Box::pin(future::ready(Err(StripeError::QueryStringSerialize(err))))
                }
                Ok(body) => Body::from_string(body),
            })
            .build();
        req.set_header("content-type", "application/x-www-form-urlencoded");
        send(&self.client, self.set_headers(req))
    }

    fn url(&self, path: &str) -> Url {
        let mut url = self.host.clone();
        url.set_path(&format!("{}/{}", self.api_root, path.trim_start_matches('/')));
        url
    }

    fn url_with_params<P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Result<Url, StripeError> {
        let mut url = self.url(path);
        let params = serde_qs::to_string(&params).map_err(StripeError::from)?;
        url.set_query(Some(&params));
        Ok(url)
    }

    fn set_headers(&self, mut req: surf::Request) -> surf::Request {
        req.set_header("authorization", &format!("Bearer {}", self.secret_key));
        if let Some(account) = &self.headers.stripe_account {
            req.set_header("stripe-account", account);
        }
        if let Some(client_id) = &self.headers.client_id {
            req.set_header("client-id", client_id);
        }
        if let Some(stripe_version) = &self.headers.stripe_version {
            req.set_header("stripe-version", stripe_version.as_str());
        }
        const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
        let user_agent: String = format!("Stripe/v3 RustBindings/{}", CRATE_VERSION);
        if let Some(app_info) = &self.app_info {
            let formatted: String = format_app_info(app_info);
            let user_agent_app_info: String =
                format!("{} {}", user_agent, formatted).trim().to_owned();
            req.set_header("user-agent", user_agent_app_info.as_str());
        } else {
            req.set_header("user-agent", user_agent.as_str());
        };

        req
    }
}

fn send<T: DeserializeOwned + Send + 'static>(
    client: &surf::Client,
    request: surf::Request,
) -> Response<T> {
    let client = client.clone(); // N.B. Client is send sync;  cloned clients share the same pool.
    Box::pin(async move {
        let mut response = client.send(request).await?;
        let status = response.status();
        let bytes = response.body_bytes().await?;
        if !status.is_success() {
            Err(serde_json::from_slice(&bytes)
                .map(|mut e: ErrorResponse| {
                    e.error.http_status = status.into();
                    StripeError::from(e.error)
                })
                .unwrap_or_else(StripeError::from))
        } else {
            serde_json::from_slice(&bytes).map_err(StripeError::from)
        }
    })
}

/// Formats a plugin's 'App Info' into a string that can be added to the end of an User-Agent string.
///
/// This formatting matches that of other libraries, and if changed then it should be changed everywhere.
fn format_app_info(info: &AppInfo) -> String {
    let formatted: String = match (&info.version, &info.url) {
        (Some(a), Some(b)) => format!("{}/{} ({})", &info.name, a, b),
        (Some(a), None) => format!("{}/{}", &info.name, a),
        (None, Some(b)) => format!("{}/{}", &info.name, b),
        _ => info.name.to_string(),
    };
    formatted
}
