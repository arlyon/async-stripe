use crate::error::{Error, ErrorResponse, RequestError};
use crate::params::{AppInfo, Headers};
use crate::resources::ApiVersion;
use futures_util::future;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use http::request::Builder as RequestBuilder;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::pin::Pin;

#[cfg(feature = "rustls-tls")]
use hyper_rustls::HttpsConnector;
#[cfg(feature = "default-tls")]
use hyper_tls::HttpsConnector;
#[cfg(all(feature = "default-tls", feature = "rustls-tls"))]
compile_error!("You must enable only one TLS implementation");
#[cfg(not(any(feature = "default-tls", feature = "rustls-tls")))]
compile_error!("You must enable at least one TLS implementation; add `features = [\"default-tls\"]` to your Cargo.toml");

type HttpClient = hyper::Client<HttpsConnector<hyper::client::HttpConnector>, hyper::Body>;

pub type Response<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn ok<T: Send + 'static>(ok: T) -> Response<T> {
    Box::pin(future::ready(Ok(ok)))
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn err<T: Send + 'static>(err: Error) -> Response<T> {
    Box::pin(future::ready(Err(err)))
}

#[derive(Clone)]
pub struct Client {
    host: String,
    client: HttpClient,
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
    pub fn from_url(scheme_host: impl Into<String>, secret_key: impl Into<String>) -> Client {
        let url = scheme_host.into();
        let host = if url.ends_with('/') { format!("{}v1", url) } else { format!("{}/v1", url) };
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build(https);
        let mut headers = Headers::default();
        // TODO: Automatically determine the latest supported api version in codegen?
        headers.stripe_version = Some(ApiVersion::V2019_09_09);
        Client {
            host,
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
        let mut req =
            RequestBuilder::new().method("GET").uri(url).body(hyper::Body::empty()).unwrap();
        *req.headers_mut() = self.headers();
        send(&self.client, req)
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
        let mut req =
            RequestBuilder::new().method("GET").uri(url).body(hyper::Body::empty()).unwrap();
        *req.headers_mut() = self.headers();
        send(&self.client, req)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        let mut req =
            RequestBuilder::new().method("DELETE").uri(url).body(hyper::Body::empty()).unwrap();
        *req.headers_mut() = self.headers();
        send(&self.client, req)
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
        let mut req =
            RequestBuilder::new().method("DELETE").uri(url).body(hyper::Body::empty()).unwrap();
        *req.headers_mut() = self.headers();
        send(&self.client, req)
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        let mut req =
            RequestBuilder::new().method("POST").uri(url).body(hyper::Body::empty()).unwrap();
        *req.headers_mut() = self.headers();
        send(&self.client, req)
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned + Send + 'static, F: serde::Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        let url = self.url(path);
        let mut req = RequestBuilder::new()
            .method("POST")
            .uri(url)
            .body(match serde_qs::to_string(&form) {
                Err(err) => return Box::pin(future::ready(Err(Error::serialize(err)))),
                Ok(body) => hyper::Body::from(body),
            })
            .unwrap();
        *req.headers_mut() = self.headers();
        req.headers_mut().insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        send(&self.client, req)
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.host, path.trim_start_matches('/'))
    }

    fn url_with_params<P: serde::Serialize>(&self, path: &str, params: P) -> Result<String, Error> {
        let params = serde_qs::to_string(&params).map_err(Error::serialize)?;
        Ok(format!("{}/{}?{}", self.host, &path[1..], params))
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(&format!("Bearer {}", self.secret_key)).unwrap(),
        );
        if let Some(account) = &self.headers.stripe_account {
            headers.insert(
                HeaderName::from_static("stripe-account"),
                HeaderValue::from_str(account).unwrap(),
            );
        }
        if let Some(client_id) = &self.headers.client_id {
            headers.insert(
                HeaderName::from_static("client-id"),
                HeaderValue::from_str(client_id).unwrap(),
            );
        }
        if let Some(stripe_version) = &self.headers.stripe_version {
            headers.insert(
                HeaderName::from_static("stripe-version"),
                HeaderValue::from_str(stripe_version.as_str()).unwrap(),
            );
        }
        const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
        let user_agent: String = format!("Stripe/v3 RustBindings/{}", CRATE_VERSION);
        if let Some(app_info) = &self.app_info {
            let formatted: String = format_app_info(app_info);
            let user_agent_app_info: String =
                format!("{} {}", user_agent, formatted).trim().to_owned();
            headers.insert(
                HeaderName::from_static("user-agent"),
                HeaderValue::from_str(user_agent_app_info.as_str()).unwrap(),
            );
        } else {
            headers.insert(
                HeaderName::from_static("user-agent"),
                HeaderValue::from_str(user_agent.as_str()).unwrap(),
            );
        };
        headers
    }
}

fn send<T: DeserializeOwned + Send + 'static>(
    client: &HttpClient,
    request: hyper::Request<hyper::Body>,
) -> Response<T> {
    let client = client.clone(); // N.B. Client is send sync;  cloned clients share the same pool.
    Box::pin(async move {
        let response = client.request(request).await?;
        let status = response.status();
        let bytes = hyper::body::to_bytes(response.into_body()).await?;
        if !status.is_success() {
            let mut err = serde_json::from_slice(&bytes).unwrap_or_else(|err| {
                let mut req = ErrorResponse { error: RequestError::default() };
                req.error.message = Some(format!("failed to deserialize error: {}", err));
                req
            });
            err.error.http_status = status.as_u16();
            Err(Error::from(err.error))?;
        }
        serde_json::from_slice(&bytes).map_err(Error::deserialize)
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
