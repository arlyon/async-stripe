use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
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
pub struct CreateFinancialConnectionsSessionAccountHolder {
    /// The ID of the Stripe account whose accounts you will retrieve.
    /// Only available when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The ID of the Stripe customer whose accounts you will retrieve.
    /// Only available when `type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    /// The ID of Account representing a customer whose accounts you will retrieve.
    /// Only available when `type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    /// Type of account holder to collect accounts for.
    #[serde(rename = "type")]
    pub type_: CreateFinancialConnectionsSessionAccountHolderType,
}
impl CreateFinancialConnectionsSessionAccountHolder {
    pub fn new(type_: impl Into<CreateFinancialConnectionsSessionAccountHolderType>) -> Self {
        Self { account: None, customer: None, customer_account: None, type_: type_.into() }
    }
}
/// Type of account holder to collect accounts for.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateFinancialConnectionsSessionAccountHolderType {
    Account,
    Customer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateFinancialConnectionsSessionAccountHolderType {
    pub fn as_str(&self) -> &str {
        use CreateFinancialConnectionsSessionAccountHolderType::*;
        match self {
            Account => "account",
            Customer => "customer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateFinancialConnectionsSessionAccountHolderType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFinancialConnectionsSessionAccountHolderType::*;
        match s {
            "account" => Ok(Account),
            "customer" => Ok(Customer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateFinancialConnectionsSessionAccountHolderType"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Filters to restrict the kinds of accounts to collect.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateFinancialConnectionsSessionFilters {
    /// Restricts the Session to subcategories of accounts that can be linked.
    /// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_subcategories:
        Option<Vec<CreateFinancialConnectionsSessionFiltersAccountSubcategories>>,
    /// List of countries from which to collect accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
}
impl CreateFinancialConnectionsSessionFilters {
    pub fn new() -> Self {
        Self { account_subcategories: None, countries: None }
    }
}
impl Default for CreateFinancialConnectionsSessionFilters {
    fn default() -> Self {
        Self::new()
    }
}
/// Restricts the Session to subcategories of accounts that can be linked.
/// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Savings,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    pub fn as_str(&self) -> &str {
        use CreateFinancialConnectionsSessionFiltersAccountSubcategories::*;
        match self {
            Checking => "checking",
            CreditCard => "credit_card",
            LineOfCredit => "line_of_credit",
            Mortgage => "mortgage",
            Savings => "savings",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFinancialConnectionsSessionFiltersAccountSubcategories::*;
        match s {
            "checking" => Ok(Checking),
            "credit_card" => Ok(CreditCard),
            "line_of_credit" => Ok(LineOfCredit),
            "mortgage" => Ok(Mortgage),
            "savings" => Ok(Savings),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateFinancialConnectionsSessionFiltersAccountSubcategories"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// To launch the Financial Connections authorization flow, create a `Session`.
/// The session’s `client_secret` can be used to launch the flow using Stripe.js.
#[derive(Clone, Debug, serde::Serialize)]
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
