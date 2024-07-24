use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveFinancialConnectionsSessionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFinancialConnectionsSessionBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a Financial Connections `Session`
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFinancialConnectionsSession<'a> {
    inner: RetrieveFinancialConnectionsSessionBuilder<'a>,
    session: &'a stripe_misc::FinancialConnectionsSessionId,
}
impl<'a> RetrieveFinancialConnectionsSession<'a> {
    /// Construct a new `RetrieveFinancialConnectionsSession`.
    pub fn new(session: &'a stripe_misc::FinancialConnectionsSessionId) -> Self {
        Self { session, inner: RetrieveFinancialConnectionsSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveFinancialConnectionsSession<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveFinancialConnectionsSession<'_> {
    type Output = stripe_misc::FinancialConnectionsSession;

    fn build(&self) -> RequestBuilder {
        let session = self.session;
        RequestBuilder::new(StripeMethod::Get, format!("/financial_connections/sessions/{session}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateFinancialConnectionsSessionBuilder<'a> {
    account_holder: CreateFinancialConnectionsSessionAccountHolder<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<CreateFinancialConnectionsSessionFilters<'a>>,
    permissions: &'a [stripe_misc::FinancialConnectionsSessionPermissions],
    #[serde(skip_serializing_if = "Option::is_none")]
    prefetch: Option<&'a [stripe_misc::FinancialConnectionsSessionPrefetch]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<&'a str>,
}
impl<'a> CreateFinancialConnectionsSessionBuilder<'a> {
    fn new(
        account_holder: CreateFinancialConnectionsSessionAccountHolder<'a>,
        permissions: &'a [stripe_misc::FinancialConnectionsSessionPermissions],
    ) -> Self {
        Self {
            account_holder,
            expand: None,
            filters: None,
            permissions,
            prefetch: None,
            return_url: None,
        }
    }
}
/// The account holder to link accounts for.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialConnectionsSessionAccountHolder<'a> {
    /// The ID of the Stripe account whose accounts will be retrieved.
    /// Should only be present if `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// The ID of the Stripe customer whose accounts will be retrieved.
    /// Should only be present if `type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Type of account holder to collect accounts for.
    #[serde(rename = "type")]
    pub type_: CreateFinancialConnectionsSessionAccountHolderType,
}
impl<'a> CreateFinancialConnectionsSessionAccountHolder<'a> {
    pub fn new(type_: CreateFinancialConnectionsSessionAccountHolderType) -> Self {
        Self { account: None, customer: None, type_ }
    }
}
/// Type of account holder to collect accounts for.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateFinancialConnectionsSessionAccountHolderType {
    Account,
    Customer,
}
impl CreateFinancialConnectionsSessionAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreateFinancialConnectionsSessionAccountHolderType::*;
        match self {
            Account => "account",
            Customer => "customer",
        }
    }
}

impl std::str::FromStr for CreateFinancialConnectionsSessionAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFinancialConnectionsSessionAccountHolderType::*;
        match s {
            "account" => Ok(Account),
            "customer" => Ok(Customer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateFinancialConnectionsSessionAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateFinancialConnectionsSessionAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateFinancialConnectionsSessionAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateFinancialConnectionsSessionAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateFinancialConnectionsSessionAccountHolderType",
            )
        })
    }
}
/// Filters to restrict the kinds of accounts to collect.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialConnectionsSessionFilters<'a> {
    /// List of countries from which to collect accounts.
    pub countries: &'a [&'a str],
}
impl<'a> CreateFinancialConnectionsSessionFilters<'a> {
    pub fn new(countries: &'a [&'a str]) -> Self {
        Self { countries }
    }
}
/// To launch the Financial Connections authorization flow, create a `Session`.
/// The session’s `client_secret` can be used to launch the flow using Stripe.js.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateFinancialConnectionsSession<'a> {
    inner: CreateFinancialConnectionsSessionBuilder<'a>,
}
impl<'a> CreateFinancialConnectionsSession<'a> {
    /// Construct a new `CreateFinancialConnectionsSession`.
    pub fn new(
        account_holder: CreateFinancialConnectionsSessionAccountHolder<'a>,
        permissions: &'a [stripe_misc::FinancialConnectionsSessionPermissions],
    ) -> Self {
        Self { inner: CreateFinancialConnectionsSessionBuilder::new(account_holder, permissions) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Filters to restrict the kinds of accounts to collect.
    pub fn filters(mut self, filters: CreateFinancialConnectionsSessionFilters<'a>) -> Self {
        self.inner.filters = Some(filters);
        self
    }
    /// List of data features that you would like to retrieve upon account creation.
    pub fn prefetch(
        mut self,
        prefetch: &'a [stripe_misc::FinancialConnectionsSessionPrefetch],
    ) -> Self {
        self.inner.prefetch = Some(prefetch);
        self
    }
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    pub fn return_url(mut self, return_url: &'a str) -> Self {
        self.inner.return_url = Some(return_url);
        self
    }
}
impl CreateFinancialConnectionsSession<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateFinancialConnectionsSession<'_> {
    type Output = stripe_misc::FinancialConnectionsSession;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/financial_connections/sessions").form(&self.inner)
    }
}
