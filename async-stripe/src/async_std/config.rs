use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use http_types::Url;
use stripe_client_core::{RequestStrategy, SharedConfigBuilder};
use stripe_shared::{AccountId, ApiVersion, ApplicationId};

use crate::async_std::Client;
use crate::StripeError;

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
            Url::from_str(&url).map_err(|err| {
                StripeError::ConfigError(format!("user-provided Stripe url is invalid: {err}"))
            })?
        } else {
            Url::from_str(DEFAULT_API_BASE).expect("is valid URL")
        };

        let user_agent = if let Some(app_info_str) = self.inner.app_info_str {
            format!("{DEFAULT_USER_AGENT} {app_info_str}")
        } else {
            DEFAULT_USER_AGENT.into()
        };

        Ok(ClientConfig {
            stripe_version: self.inner.stripe_version,
            user_agent,
            client_id: self.inner.client_id,
            account_id: self.inner.account_id,
            request_strategy: self.inner.request_strategy.unwrap_or(RequestStrategy::Once),
            secret: self.inner.secret,
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
}

/// A finalized client configuration.
#[derive(Clone)]
pub struct ClientConfig {
    pub stripe_version: ApiVersion,
    pub user_agent: String,
    pub client_id: Option<ApplicationId>,
    pub account_id: Option<AccountId>,
    pub request_strategy: RequestStrategy,
    pub secret: String,
    pub api_base: Url,
}

impl ClientConfig {
    pub fn to_headers_array(&self) -> [(&str, Option<&str>); 4] {
        [
            ("Client-Id", self.client_id.as_deref()),
            ("Stripe-Account", self.account_id.as_deref()),
            ("Stripe-Version", Some(self.stripe_version.as_str())),
            ("User-Agent", Some(&self.user_agent)),
        ]
    }
}

// Don't print the secret!
impl Debug for ClientConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ClientConfig");
        s.field("request_strategy", &self.request_strategy);
        s.field("client_id", &self.client_id);
        s.field("account_id", &self.account_id);
        s.field("api_base", &self.api_base);
        s.field("user_agent", &self.user_agent);
        s.field("stripe_version", &self.stripe_version);
        s.finish()
    }
}
