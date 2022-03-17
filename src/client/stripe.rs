use std::future;

use http_types::{Body, Method, Request, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    client::{request_strategy::RequestStrategy, BaseClient, Response},
    params::AppInfo,
    ApiVersion, Headers, StripeError,
};

static USER_AGENT: &str = concat!("Stripe/v3 RustBindings/", env!("CARGO_PKG_VERSION"));

pub struct Client {
    client: crate::client::BaseClient,
    secret_key: String,
    headers: Headers,
    strategy: RequestStrategy,
    app_info: Option<AppInfo>,
    api_base: Url,
    api_root: String,
}

impl Client {
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self::from_url("https://api.stripe.com/", secret_key)
    }

    pub fn from_url<'a>(url: impl Into<&'a str>, secret_key: impl Into<String>) -> Self {
        Client {
            client: BaseClient::new(),
            secret_key: secret_key.into(),
            headers: Headers {
                stripe_version: Some(ApiVersion::V2020_08_27),
                user_agent: USER_AGENT.to_string(),
                ..Default::default()
            },
            strategy: RequestStrategy::Once,
            app_info: None,
            api_base: Url::parse(url.into()).expect("invalid url"),
            api_root: "v1".to_string(),
        }
    }

    /// Clones a new client with different headers.
    ///
    /// This is the recommended way to send requests for many different Stripe accounts
    /// or with different Meta, Extra, and Expand headers while using the same secret key.
    pub fn with_headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn with_app_info(
        mut self,
        name: String,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        let app_info = AppInfo { name, version, url };
        self.headers.user_agent = format!("{}/{}", USER_AGENT, app_info.to_string());
        self.app_info = Some(app_info);
        self
    }

    /// Make a `GET` http request with just a path
    pub fn get<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Get, url), &self.strategy)
    }

    /// Make a `GET` http request with url query parameters
    pub fn get_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(err) => return Box::pin(future::ready(Err(err))),
            Ok(ok) => ok,
        };
        self.client.execute::<T>(self.create_request(Method::Get, url), &self.strategy)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Delete, url), &self.strategy)
    }

    /// Make a `DELETE` http request with url query parameters
    pub fn delete_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(err) => return Box::pin(future::ready(Err(err))),
            Ok(ok) => ok,
        };
        self.client.execute::<T>(self.create_request(Method::Delete, url), &self.strategy)
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Post, url), &self.strategy)
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned + Send + 'static, F: Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        let url = self.url(path);
        let mut req = self.create_request(Method::Post, url);
        req.set_body(match serde_qs::to_string(&form) {
            Err(err) => {
                return Box::pin(future::ready(Err(StripeError::QueryStringSerialize(err))))
            }
            Ok(body) => Body::from_string(body),
        });
        req.insert_header("content-type", "application/x-www-form-urlencoded");
        self.client.execute::<T>(req, &self.strategy)
    }

    fn url(&self, path: &str) -> Url {
        let mut url = self.api_base.clone();
        url.set_path(&format!("{}/{}", self.api_root, path.trim_start_matches('/')));
        url
    }

    fn url_with_params<P: Serialize>(&self, path: &str, params: P) -> Result<Url, StripeError> {
        let mut url = self.url(path);
        let params = serde_qs::to_string(&params).map_err(StripeError::from)?;
        url.set_query(Some(&params));
        Ok(url)
    }

    fn create_request(&self, method: Method, url: Url) -> Request {
        let mut req = Request::new(method, url);
        req.insert_header("authorization", &format!("Bearer {}", self.secret_key));

        for (key, value) in self.headers.into_iter().filter_map(|(k, v)| v.map(|v| (k, v))) {
            req.insert_header(key, value);
        }

        req
    }
}
