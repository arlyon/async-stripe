use hyper::http::{HeaderValue, Uri};
use stripe_client_core::{RequestStrategy, SharedConfigBuilder};
use stripe_shared::{AccountId, ApplicationId};

use crate::{Client, StripeError};

static DEFAULT_USER_AGENT: &str = concat!("Stripe/v1 RustBindings/", env!("CARGO_PKG_VERSION"));
const DEFAULT_API_BASE: &str = "https://api.stripe.com/";

/// Configuration for a Stripe client.
///
/// This builder allows customizing request behavior, headers sent to Stripe, and endpoint
/// targeted. Some defaults can be overridden on a per-request basis.
#[derive(Clone, Debug)]
pub struct ClientBuilder {
    inner: SharedConfigBuilder,
}

impl ClientBuilder {
    /// Create a new `ClientBuilder` with the given secret key.
    pub fn new(secret: impl Into<String>) -> Self {
        Self { inner: SharedConfigBuilder::new(secret) }
    }

    /// Set the Stripe client id for the client. This will be sent to stripe
    /// with the "client-id" header.
    pub fn client_id(mut self, client_id: ApplicationId) -> Self {
        self.inner = self.inner.client_id(client_id);
        self
    }

    /// Set the default Stripe account id used for the client. This will be sent to stripe
    /// with the "stripe-account" header, unless it is overridden when customizing
    /// a request.
    pub fn account_id(mut self, account_id: AccountId) -> Self {
        self.inner = self.inner.account_id(account_id);
        self
    }

    /// Set the default `RequestStrategy` used when making requests.
    pub fn request_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.inner = self.inner.request_strategy(strategy);
        self
    }

    /// Create a new client pointed at a specific URL. This is useful for testing.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.inner = self.inner.url(url);
        self
    }

    /// Set the application info for the client.
    ///
    /// It is recommended that applications set this so that
    /// Stripe is able to understand usage patterns from your
    /// user agent.
    pub fn app_info(
        mut self,
        name: impl Into<String>,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        self.inner = self.inner.app_info(name, version, url);
        self
    }

    fn try_into_config(self) -> Result<ClientConfig, StripeError> {
        let api_base = if let Some(url) = self.inner.api_base {
            Uri::try_from(url).map_err(|err| {
                StripeError::ConfigError(format!("user-provided Stripe url is invalid: {err}"))
            })?
        } else {
            Uri::from_static(DEFAULT_API_BASE)
        };

        let user_agent_header = if let Some(app_info_str) = self.inner.app_info_str {
            HeaderValue::try_from(format!("{DEFAULT_USER_AGENT} {app_info_str}"))
                .map_err(|_| cons_header_err("app_info"))?
        } else {
            HeaderValue::from_static(DEFAULT_USER_AGENT)
        };

        let mut secret = HeaderValue::try_from(format!("Bearer {}", self.inner.secret))
            .map_err(|_| cons_header_err("secret"))?;
        secret.set_sensitive(true);
        Ok(ClientConfig {
            stripe_version: HeaderValue::from_str(self.inner.stripe_version.as_str())
                .expect("all stripe versions produce valid header values"),
            user_agent: user_agent_header,
            client_id: self
                .inner
                .client_id
                .map(|id| HeaderValue::try_from(id.to_string()))
                .transpose()
                .map_err(|_| cons_header_err("client_id"))?,
            account_id: self
                .inner
                .account_id
                .map(|id| HeaderValue::try_from(id.to_string()))
                .transpose()
                .map_err(|_| cons_header_err("account_id"))?,
            request_strategy: self.inner.request_strategy.unwrap_or(RequestStrategy::Once),
            secret,
            api_base,
        })
    }

    /// Builds a Stripe `client`.
    ///
    /// # Errors
    /// This method errors if any of the specified configuration is invalid.
    pub fn build(self) -> Result<Client, StripeError> {
        Ok(Client::from_config(self.try_into_config()?))
    }

    /// Builds a Stripe `client` for making blocking API calls.
    ///
    /// # Errors
    /// This method errors if any of the specified configuration is invalid.
    ///
    /// # Panics
    /// This method panics if called from within an async runtime.
    #[cfg(feature = "blocking")]
    pub fn build_sync(self) -> Result<crate::blocking::Client, StripeError> {
        Ok(crate::blocking::Client::from_async(self.build()?))
    }
}

fn cons_header_err(config_name: &'static str) -> StripeError {
    StripeError::ClientError(format!("`{config_name}` can only include visible ASCII characters"))
}

/// A validated client configuration. All configuration types are carefully chosen to be
/// cheaply clonable so that the client can be cheaply cloned.
#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub stripe_version: HeaderValue,
    pub user_agent: HeaderValue,
    pub client_id: Option<HeaderValue>,
    pub account_id: Option<HeaderValue>,
    pub request_strategy: RequestStrategy,
    // NB: This `HeaderValue` is marked as sensitive, so it won't be debug printed.
    pub secret: HeaderValue,
    pub api_base: Uri,
}
