use std::fmt;
use std::fmt::{Display, Formatter};

use stripe_shared::version::VERSION;
use stripe_shared::{AccountId, ApiVersion, ApplicationId};

use crate::RequestStrategy;

/// Shared configuration utilities for implementing a Stripe client.
///
/// This is meant for internal use when implementing compatible clients,
/// so it may be more unstable with respect to semver.

// The disclaimer above was written to justify the semver hazard of keeping all the fields here public.
// This is not necessary, but writing accessors is tricky because configs using this
// internally want to take ownership of each field to avoid unnecessary clones.
#[derive(Clone)]
pub struct SharedConfigBuilder {
    /// The stripe version being used.
    pub stripe_version: ApiVersion,
    /// The user-provided part of the user-agent we'll use.
    pub app_info_str: Option<String>,
    /// The client id to send requests with.
    pub client_id: Option<ApplicationId>,
    /// The account id to send requests with.
    pub account_id: Option<AccountId>,
    /// The default request strategy to use.,
    pub request_strategy: Option<RequestStrategy>,
    /// The secret key for authorizing requests.
    pub secret: String,
    /// The base URL to send requests to.
    pub api_base: Option<String>,
}

impl SharedConfigBuilder {
    /// Create a new `SharedConfigBuilder` with the given secret key.
    pub fn new(secret: impl Into<String>) -> Self {
        let secret = secret.into();

        // some basic sanity checks
        // TODO: maybe a full-blown type here rather than a warning?
        if secret.trim() != secret || !secret.starts_with("sk_") {
            tracing::warn!("suspiciously formatted secret key")
        }

        Self {
            stripe_version: VERSION,
            app_info_str: None,
            client_id: None,
            account_id: None,
            request_strategy: None,
            secret,
            api_base: None,
        }
    }

    /// Set the Stripe client id for the client. This will be sent to stripe
    /// with the "client-id" header.
    pub fn client_id(mut self, client_id: ApplicationId) -> Self {
        self.client_id = Some(client_id);
        self
    }

    /// Set the default Stripe account id used for the client. This will be sent to stripe
    /// with the "stripe-account" header, unless it is overridden when customizing
    /// a request.
    pub fn account_id(mut self, account_id: AccountId) -> Self {
        self.account_id = Some(account_id);
        self
    }

    /// Sets the default `RequestStrategy` used when making requests.
    pub fn request_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.request_strategy = Some(strategy);
        self
    }

    /// Create a new client pointed at a specific URL. This is useful for testing.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.api_base = Some(url.into());
        self
    }

    /// Set the application info for the client.
    pub fn app_info(
        mut self,
        name: impl Into<String>,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        self.app_info_str = Some(AppInfo { name: name.into(), url, version }.to_string());
        self
    }
}

struct AppInfo {
    name: String,
    url: Option<String>,
    version: Option<String>,
}

impl Display for AppInfo {
    /// Formats a plugin's 'App Info' into a string that can be added to the end of a User-Agent string.
    ///
    /// This formatting matches that of other libraries, and if changed then it should be changed everywhere.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        match (&self.version, &self.url) {
            (Some(a), Some(b)) => write!(f, "{name}/{a} ({b})"),
            (Some(a), None) => write!(f, "{name}/{a}"),
            (None, Some(b)) => write!(f, "{name} ({b})"),
            _ => f.write_str(name),
        }
    }
}

// Manual implementation so we don't print the secret!
impl fmt::Debug for SharedConfigBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("SharedConfigBuilder");
        builder.field("request_strategy", &self.request_strategy);
        builder.field("client_id", &self.client_id);
        builder.field("account_id", &self.account_id);
        builder.field("app_info_str", &self.app_info_str);
        if let Some(api_base) = &self.api_base {
            builder.field("api_base", api_base);
        }
        builder.field("stripe_version", &self.stripe_version);
        builder.finish()
    }
}

/// Per-request configuration overrides.
#[derive(Debug)]
#[non_exhaustive]
pub struct ConfigOverride {
    /// Use a particular account id, instead of the client default.
    pub account_id: Option<AccountId>,
    /// Use a particular `RequestStrategy`, instead of the client default.
    pub request_strategy: Option<RequestStrategy>,
}

impl ConfigOverride {
    pub(crate) fn new() -> Self {
        Self { account_id: None, request_strategy: None }
    }
}
