use std::future::{self, Future};
use std::pin::Pin;

use hyper::{
    client::HttpConnector,
    header::{HeaderMap, HeaderName, HeaderValue},
    Body, Request,
};
use serde::de::DeserializeOwned;

use crate::error::{ErrorResponse, StripeError};
use crate::params::{AppInfo, Headers};
use crate::resources::ApiVersion;

static USER_AGENT: &str = concat!("Stripe/v3 RustBindings/", env!("CARGO_PKG_VERSION"));

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
        let https = connector::create();
        let client = hyper::Client::builder().build(https);

        // TODO: Automatically determine the latest supported api version in codegen?
        let headers =
            Headers { stripe_version: Some(ApiVersion::V2020_08_27), ..Default::default() };

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
        let mut req = Request::builder()
            .method("GET")
            .uri(url)
            .body(Body::empty())
            .expect("request is correct");
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
        let mut req = Request::builder()
            .method("GET")
            .uri(url)
            .body(Body::empty())
            .expect("request is correct");
        *req.headers_mut() = self.headers();
        send(&self.client, req)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        let mut req = Request::builder()
            .method("DELETE")
            .uri(url)
            .body(Body::empty())
            .expect("request is correct");
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
        let mut req = Request::builder()
            .method("DELETE")
            .uri(url)
            .body(Body::empty())
            .expect("request is correct");
        *req.headers_mut() = self.headers();
        send(&self.client, req)
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        let mut req = Request::builder()
            .method("POST")
            .uri(url)
            .body(Body::empty())
            .expect("request is correct");
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
        let mut req = Request::builder()
            .method("POST")
            .uri(url)
            .body(match serde_qs::to_string(&form) {
                Err(err) => {
                    return Box::pin(future::ready(Err(StripeError::QueryStringSerialize(err))))
                }
                Ok(body) => Body::from(body),
            })
            .expect("request is correct");
        *req.headers_mut() = self.headers();
        req.headers_mut().insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        send(&self.client, req)
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.host, path.trim_start_matches('/'))
    }

    fn url_with_params<P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Result<String, StripeError> {
        let params = serde_qs::to_string(&params).map_err(StripeError::from)?;
        Ok(format!("{}/{}?{}", self.host, &path[1..], params))
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(&format!("Bearer {}", self.secret_key))
                .expect("secret key is valid header value"),
        );
        if let Some(account) = &self.headers.stripe_account {
            headers.insert(
                HeaderName::from_static("stripe-account"),
                HeaderValue::from_str(account).expect("stripe account is valid header value"),
            );
        }
        if let Some(client_id) = &self.headers.client_id {
            headers.insert(
                HeaderName::from_static("client-id"),
                HeaderValue::from_str(client_id).expect("client id valid header value"),
            );
        }
        if let Some(stripe_version) = &self.headers.stripe_version {
            headers.insert(
                HeaderName::from_static("stripe-version"),
                HeaderValue::from_str(stripe_version.as_str())
                    .expect("stripe version is valid header value"),
            );
        }
        if let Some(app_info) = &self.app_info {
            let user_agent_app_info = format!("{} {}", USER_AGENT, format_app_info(app_info));
            headers.insert(
                HeaderName::from_static("user-agent"),
                HeaderValue::from_str(user_agent_app_info.trim())
                    .expect("app info is valid header value"),
            );
        } else {
            headers.insert(
                HeaderName::from_static("user-agent"),
                HeaderValue::from_static(USER_AGENT),
            );
        };
        headers
    }
}

fn send<T: DeserializeOwned + Send + 'static>(
    client: &HttpClient,
    request: hyper::Request<Body>,
) -> Response<T> {
    let client = client.clone(); // N.B. Client is send sync;  cloned clients share the same pool.
    Box::pin(async move {
        let bytes = send_inner(&client, request).await?;
        let mut jd = serde_json::Deserializer::from_slice(&bytes);
        let result : Result<T, _> = serde_path_to_error::deserialize(&mut jd);
        if let Err(err) = result
        {
            println!("{:#?}", err);
        }
        serde_json::from_slice(&bytes).map_err(StripeError::from)
    })
}

async fn send_inner(
    client: &HttpClient,
    request: hyper::Request<Body>,
) -> Result<hyper::body::Bytes, StripeError> {
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
