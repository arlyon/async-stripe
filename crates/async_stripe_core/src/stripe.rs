use http_types::{Body, Method, Request, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    base_client::BaseClient,
    error::StripeError,
    ids::{AccountId, ApplicationId},
    params::{AppInfo, Headers},
    request_strategy::RequestStrategy,
    version::ApiVersion,
};

static USER_AGENT: &str = concat!("Stripe/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

#[derive(Clone)]
pub struct Client<C: BaseClient> {
    client: C,
    secret_key: String,
    headers: Headers,
    strategy: RequestStrategy,
    app_info: Option<AppInfo>,
    api_base: Url,
    api_root: String,
}

impl<C: BaseClient> Client<C> {
    /// Create a new account with the given secret key.
    pub fn new(secret_key: impl Into<String>, client: C) -> Self {
        Self::from_url("https://api.stripe.com/", secret_key, client)
    }

    /// Create a new account pointed at a specific URL. This is useful for testing.
    pub fn from_url<'a>(url: impl Into<&'a str>, secret_key: impl Into<String>, client: C) -> Self {
        Client {
            client,
            secret_key: secret_key.into(),
            headers: Headers {
                stripe_version: ApiVersion::V2020_08_27,
                user_agent: USER_AGENT.to_string(),
                client_id: None,
                stripe_account: None,
            },
            strategy: RequestStrategy::Once,
            app_info: None,
            api_base: Url::parse(url.into()).expect("invalid url"),
            api_root: "v1".to_string(),
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
    pub fn with_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set the application info for the client.
    ///
    /// It is recommended that applications set this so that
    /// stripe is able to undestand usage patterns from your
    /// user agent.
    pub fn with_app_info(
        mut self,
        name: String,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        let app_info = AppInfo { name, version, url };
        self.headers.user_agent = format!("{} {}", USER_AGENT, app_info.to_string());
        self.app_info = Some(app_info);
        self
    }

    pub fn ok<T>(&self, item: T) -> C::Response<T> {
        self.client.ok(item)
    }

    pub fn err<T>(&self, err: StripeError) -> C::Response<T> {
        self.client.err(err)
    }

    pub fn map<T, U, F: FnOnce(T) -> U>(&self, a: C::Response<T>, fun: F) -> C::Response<U> {
        self.client.map(a, fun)
    }

    /// Make a `GET` http request with just a path
    pub fn get<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> C::Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Get, url), &self.strategy)
    }

    /// Make a `GET` http request with url query parameters

    pub fn get_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> C::Response<T> {
        let url = match self.url_with_params(path, params) {
            Ok(u) => u,
            Err(e) => return self.client.err(e),
        };
        self.client.execute::<T>(self.create_request(Method::Get, url), &self.strategy)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> C::Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Delete, url), &self.strategy)
    }

    /// Make a `DELETE` http request with url query parameters
    pub fn delete_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> C::Response<T> {
        let url = match self.url_with_params(path, params) {
            Ok(u) => u,
            Err(e) => return self.client.err(e),
        };
        self.client.execute::<T>(self.create_request(Method::Delete, url), &self.strategy)
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> C::Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Post, url), &self.strategy)
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned + Send + 'static, F: Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> C::Response<T> {
        let url = self.url(path);
        let mut req = self.create_request(Method::Post, url);

        let mut params_buffer = Vec::new();
        let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
        if let Err(qs_ser_err) = serde_path_to_error::serialize(&form, qs_ser) {
            return self.client.err(StripeError::QueryStringSerialize(qs_ser_err));
        }

        let body = std::str::from_utf8(params_buffer.as_slice())
            .expect("Unable to extract string from params_buffer")
            .to_string();

        req.set_body(Body::from_string(body));

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

        let mut params_buffer = Vec::new();
        let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
        serde_path_to_error::serialize(&params, qs_ser).map_err(StripeError::from)?;

        let params = std::str::from_utf8(params_buffer.as_slice())
            .expect("Unable to extract string from params_buffer")
            .to_string();

        url.set_query(Some(&params));
        Ok(url)
    }

    fn create_request(&self, method: Method, url: Url) -> Request {
        let mut req = Request::new(method, url);
        req.insert_header("authorization", &format!("Bearer {}", self.secret_key));

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
