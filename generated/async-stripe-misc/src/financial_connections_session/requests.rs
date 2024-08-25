use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveFinancialConnectionsSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveFinancialConnectionsSessionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a Financial Connections `Session`
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveFinancialConnectionsSession {
    inner: RetrieveFinancialConnectionsSessionBuilder,
    session: stripe_misc::FinancialConnectionsSessionId,
}
impl RetrieveFinancialConnectionsSession {
    /// Construct a new `RetrieveFinancialConnectionsSession`.
    pub fn new(session: impl Into<stripe_misc::FinancialConnectionsSessionId>) -> Self {
        Self { session: session.into(), inner: RetrieveFinancialConnectionsSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveFinancialConnectionsSession {
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

impl StripeRequest for RetrieveFinancialConnectionsSession {
    type Output = stripe_misc::FinancialConnectionsSession;

    fn build(&self) -> RequestBuilder {
        let session = &self.session;
        RequestBuilder::new(StripeMethod::Get, format!("/financial_connections/sessions/{session}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateFinancialConnectionsSessionBuilder {
    account_holder: CreateFinancialConnectionsSessionAccountHolder,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<CreateFinancialConnectionsSessionFilters>,
    permissions: Vec<stripe_misc::FinancialConnectionsSessionPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefetch: Option<Vec<stripe_misc::FinancialConnectionsSessionPrefetch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
}
impl CreateFinancialConnectionsSessionBuilder {
    fn new(
        account_holder: impl Into<CreateFinancialConnectionsSessionAccountHolder>,
        permissions: impl Into<Vec<stripe_misc::FinancialConnectionsSessionPermissions>>,
    ) -> Self {
        Self {
            account_holder: account_holder.into(),
            expand: None,
            filters: None,
            permissions: permissions.into(),
            prefetch: None,
            return_url: None,
        }
    }
}
/// The account holder to link accounts for.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateFinancialConnectionsSessionAccountHolder {
    /// The ID of the Stripe account whose accounts will be retrieved.
    /// Should only be present if `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The ID of the Stripe customer whose accounts will be retrieved.
    /// Should only be present if `type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    /// Type of account holder to collect accounts for.
    #[serde(rename = "type")]
    pub type_: CreateFinancialConnectionsSessionAccountHolderType,
}
impl CreateFinancialConnectionsSessionAccountHolder {
    pub fn new(type_: impl Into<CreateFinancialConnectionsSessionAccountHolderType>) -> Self {
        Self { account: None, customer: None, type_: type_.into() }
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateFinancialConnectionsSessionFilters {
    /// List of countries from which to collect accounts.
    pub countries: Vec<String>,
}
impl CreateFinancialConnectionsSessionFilters {
    pub fn new(countries: impl Into<Vec<String>>) -> Self {
        Self { countries: countries.into() }
    }
}
/// To launch the Financial Connections authorization flow, create a `Session`.
/// The session’s `client_secret` can be used to launch the flow using Stripe.js.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateFinancialConnectionsSession {
    inner: CreateFinancialConnectionsSessionBuilder,
}
impl CreateFinancialConnectionsSession {
    /// Construct a new `CreateFinancialConnectionsSession`.
    pub fn new(
        account_holder: impl Into<CreateFinancialConnectionsSessionAccountHolder>,
        permissions: impl Into<Vec<stripe_misc::FinancialConnectionsSessionPermissions>>,
    ) -> Self {
        Self {
            inner: CreateFinancialConnectionsSessionBuilder::new(
                account_holder.into(),
                permissions.into(),
            ),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Filters to restrict the kinds of accounts to collect.
    pub fn filters(mut self, filters: impl Into<CreateFinancialConnectionsSessionFilters>) -> Self {
        self.inner.filters = Some(filters.into());
        self
    }
    /// List of data features that you would like to retrieve upon account creation.
    pub fn prefetch(
        mut self,
        prefetch: impl Into<Vec<stripe_misc::FinancialConnectionsSessionPrefetch>>,
    ) -> Self {
        self.inner.prefetch = Some(prefetch.into());
        self
    }
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.inner.return_url = Some(return_url.into());
        self
    }
}
impl CreateFinancialConnectionsSession {
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

impl StripeRequest for CreateFinancialConnectionsSession {
    type Output = stripe_misc::FinancialConnectionsSession;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/financial_connections/sessions").form(&self.inner)
    }
}
