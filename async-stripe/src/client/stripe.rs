// Necessary under tokio-blocking since `Response` is a type alias to a `Result`
#![allow(clippy::missing_errors_doc)]
use http_types::{Body, Method, Request, Url};
use miniserde::Deserialize;
use serde::Serialize;
use stripe_shared::version::VERSION;

use crate::client::headers::{AppInfo, Headers};
use crate::{
    client::{request_strategy::RequestStrategy, BaseClient, Response},
    config::err,
    StripeError,
};
use crate::{AccountId, ApplicationId};

static USER_AGENT: &str = concat!("Stripe/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

#[derive(Clone)]
pub struct Client {
    client: BaseClient,
    secret_key: String,
    headers: Headers,
    strategy: RequestStrategy,
    api_base: Url,
}

impl Client {
    /// Create a new account with the given secret key.
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self::from_url("https://api.stripe.com/", secret_key)
    }

    /// Create a new account pointed at a specific URL. This is useful for testing.
    ///
    /// # Panics
    /// If the url can't be parsed
    pub fn from_url<'a>(url: impl Into<&'a str>, secret_key: impl Into<String>) -> Self {
        Client {
            client: BaseClient::new(),
            secret_key: secret_key.into(),
            headers: Headers {
                stripe_version: VERSION,
                user_agent: USER_AGENT.to_string(),
                client_id: None,
                stripe_account: None,
            },
            strategy: RequestStrategy::Once,
            api_base: Url::parse(url.into()).expect("invalid url"),
        }
    }

    /// Set the client id for the client.
    pub fn with_client_id(mut self, id: ApplicationId) -> Self {
        self.headers.client_id = Some(id);
        self
    }

    /// Set the stripe account for the client.
    pub fn with_stripe_account(mut self, id: AccountId) -> Self {
        self.headers.stripe_account = Some(id);
        self
    }

    /// Set the request strategy for the client.
    ///
    /// Note: the client is cheap to clone so if you require a new client
    ///       temporarily with a new strategy you can simply clone it
    ///       and keep going.
    ///
    /// ```no_run
    /// use stripe::RequestStrategy;
    /// let client = stripe::Client::new("sk_test_123");
    /// let idempotent_client = client
    ///     .clone()
    ///     .with_strategy(RequestStrategy::Idempotent("my-key".to_string()));
    /// ```
    pub fn with_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set the application info for the client.
    ///
    /// It is recommended that applications set this so that
    /// stripe is able to understand usage patterns from your
    /// user agent.
    pub fn with_app_info(
        mut self,
        name: String,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        let app_info = AppInfo { name, version, url };
        self.headers.user_agent = format!("{USER_AGENT} {app_info}");
        self
    }

    /// Make a `GET` http request with just a path
    pub fn get<T: Deserialize + Send + 'static>(&self, path: &str) -> Response<T> {
        self.send(path, Method::Get)
    }

    /// Make a `GET` http request with url query parameters
    pub fn get_query<T: Deserialize + Send + 'static, P: Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(e) => return err(e),
            Ok(ok) => ok,
        };
        self.client.execute::<T>(self.create_request(Method::Get, url), &self.strategy)
    }

    pub fn send<T: Deserialize + Send + 'static>(&self, path: &str, method: Method) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(method, url), &self.strategy)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: Deserialize + Send + 'static>(&self, path: &str) -> Response<T> {
        self.send(path, Method::Delete)
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: Deserialize + Send + 'static>(&self, path: &str) -> Response<T> {
        self.send(path, Method::Post)
    }

    /// Make a `POST` http request with urlencoded body
    ///
    /// # Panics
    /// If the form is not serialized to an utf8 string.
    pub fn post_form<T: DeserializeOwned + Send + 'static, F: Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        self.send_form(path, form, Method::Post)
    }

    /// Make a `DELETE` http request with urlencoded body
    pub fn delete_form<T: Deserialize + Send + 'static, F: Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        self.send_form(path, form, Method::Delete)
    }

    /// Make an http request with urlencoded body
    pub fn send_form<T: Deserialize + Send + 'static, F: Serialize>(
        &self,
        path: &str,
        form: F,
        method: Method,
    ) -> Response<T> {
        let url = self.url(path);
        let mut req = self.create_request(method, url);

        // We control the types here, so this should never fail
        let body = serde_qs::to_string(&form).expect("serialization should work");
        req.set_body(Body::from_string(body));

        req.insert_header("content-type", "application/x-www-form-urlencoded");
        self.client.execute::<T>(req, &self.strategy)
    }

    fn url(&self, path: &str) -> Url {
        let mut url = self.api_base.clone();
        url.set_path(&format!("v1/{}", path.trim_start_matches('/')));
        url
    }

    fn url_with_params<P: Serialize>(&self, path: &str, params: P) -> Result<Url, StripeError> {
        let mut url = self.url(path);

        // We control the types here, so this should never fail
        let query = serde_qs::to_string(&params).expect("serialization should work");
        url.set_query(Some(&query));
        Ok(url)
    }

    fn create_request(&self, method: Method, url: Url) -> Request {
        let mut req = Request::new(method, url);
        req.insert_header("authorization", format!("Bearer {}", self.secret_key));

        for (key, value) in self.headers.to_array().iter().filter_map(|(k, v)| v.map(|v| (*k, v))) {
            req.insert_header(key, value);
        }

        req
    }
}

#[cfg(test)]
mod test {
    //! Ensures our user agent matches the format of the other stripe clients.
    //!
    //! See: <https://github.com/stripe/stripe-python/blob/3b917dc4cec6a3cccfd46961e05fe7b55c6bee87/stripe/api_requestor.py#L241>

    use super::Client;

    #[test]
    fn user_agent_base() {
        let client = Client::new("sk_test_12345");

        assert_eq!(
            client.headers.user_agent,
            format!("Stripe/v1 RustBindings/{}", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn user_agent_minimal_app_info() {
        let client =
            Client::new("sk_test_12345").with_app_info("sick-new-startup".to_string(), None, None);

        assert_eq!(
            client.headers.user_agent,
            format!("Stripe/v1 RustBindings/{} sick-new-startup", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn user_agent_all() {
        let client = Client::new("sk_test_12345").with_app_info(
            "sick-new-startup".to_string(),
            Some("0.1.0".to_string()),
            Some("https://sick-startup.io".to_string()),
        );

        assert_eq!(
            client.headers.user_agent,
            format!(
                "Stripe/v1 RustBindings/{} sick-new-startup/0.1.0 (https://sick-startup.io)",
                env!("CARGO_PKG_VERSION")
            )
        );
    }
}
