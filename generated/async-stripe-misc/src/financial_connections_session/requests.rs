use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveFinancialConnectionsSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveFinancialConnectionsSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveFinancialConnectionsSessionBuilder").finish_non_exhaustive()
    }
}
impl RetrieveFinancialConnectionsSessionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a Financial Connections `Session`
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveFinancialConnectionsSession {
    inner: RetrieveFinancialConnectionsSessionBuilder,
    session: stripe_misc::FinancialConnectionsSessionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveFinancialConnectionsSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveFinancialConnectionsSession").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateFinancialConnectionsSessionBuilder {
    account_holder: CreateFinancialConnectionsSessionAccountHolder,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<CreateFinancialConnectionsSessionFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<CreateFinancialConnectionsSessionLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_entry: Option<CreateFinancialConnectionsSessionManualEntry>,
    permissions: Vec<stripe_shared::Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefetch: Option<Vec<stripe_shared::Prefetch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateFinancialConnectionsSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateFinancialConnectionsSessionBuilder").finish_non_exhaustive()
    }
}
impl CreateFinancialConnectionsSessionBuilder {
    fn new(
        account_holder: impl Into<CreateFinancialConnectionsSessionAccountHolder>,
        permissions: impl Into<Vec<stripe_shared::Permissions>>,
    ) -> Self {
        Self {
            account_holder: account_holder.into(),
            expand: None,
            filters: None,
            limits: None,
            manual_entry: None,
            permissions: permissions.into(),
            prefetch: None,
            return_url: None,
        }
    }
}
/// The account holder to link accounts for.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
    pub type_: stripe_shared::BankConnectionsResourceAccountholderType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateFinancialConnectionsSessionAccountHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateFinancialConnectionsSessionAccountHolder").finish_non_exhaustive()
    }
}
impl CreateFinancialConnectionsSessionAccountHolder {
    pub fn new(type_: impl Into<stripe_shared::BankConnectionsResourceAccountholderType>) -> Self {
        Self { account: None, customer: None, customer_account: None, type_: type_.into() }
    }
}
/// Filters to restrict the kinds of accounts to collect.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateFinancialConnectionsSessionFilters {
    /// Restricts the Session to subcategories of accounts that can be linked.
    /// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_subcategories: Option<Vec<stripe_shared::AccountSubcategories>>,
    /// List of countries from which to collect accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    /// Whether the session should require payment method support and successful account number retrieval before completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_payment_method_support: Option<stripe_shared::RequirePaymentMethodSupport>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateFinancialConnectionsSessionFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateFinancialConnectionsSessionFilters").finish_non_exhaustive()
    }
}
impl CreateFinancialConnectionsSessionFilters {
    pub fn new() -> Self {
        Self { account_subcategories: None, countries: None, require_payment_method_support: None }
    }
}
impl Default for CreateFinancialConnectionsSessionFilters {
    fn default() -> Self {
        Self::new()
    }
}
/// Settings for configuring Session-specific limits.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateFinancialConnectionsSessionLimits {
    /// The number of accounts that can be linked in this Session.
    /// Pass an empty value to allow any number of accounts.
    pub accounts: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateFinancialConnectionsSessionLimits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateFinancialConnectionsSessionLimits").finish_non_exhaustive()
    }
}
impl CreateFinancialConnectionsSessionLimits {
    pub fn new(accounts: impl Into<i64>) -> Self {
        Self { accounts: accounts.into() }
    }
}
/// Customize manual entry behavior
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateFinancialConnectionsSessionManualEntry {
    /// How manual entry should be handled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<stripe_shared::Mode>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateFinancialConnectionsSessionManualEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateFinancialConnectionsSessionManualEntry").finish_non_exhaustive()
    }
}
impl CreateFinancialConnectionsSessionManualEntry {
    pub fn new() -> Self {
        Self { mode: None }
    }
}
impl Default for CreateFinancialConnectionsSessionManualEntry {
    fn default() -> Self {
        Self::new()
    }
}
/// To launch the Financial Connections authorization flow, create a `Session`.
/// The session’s `client_secret` can be used to launch the flow using Stripe.js.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateFinancialConnectionsSession {
    inner: CreateFinancialConnectionsSessionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateFinancialConnectionsSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateFinancialConnectionsSession").finish_non_exhaustive()
    }
}
impl CreateFinancialConnectionsSession {
    /// Construct a new `CreateFinancialConnectionsSession`.
    pub fn new(
        account_holder: impl Into<CreateFinancialConnectionsSessionAccountHolder>,
        permissions: impl Into<Vec<stripe_shared::Permissions>>,
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
    /// Settings for configuring Session-specific limits.
    pub fn limits(mut self, limits: impl Into<CreateFinancialConnectionsSessionLimits>) -> Self {
        self.inner.limits = Some(limits.into());
        self
    }
    /// Customize manual entry behavior
    pub fn manual_entry(
        mut self,
        manual_entry: impl Into<CreateFinancialConnectionsSessionManualEntry>,
    ) -> Self {
        self.inner.manual_entry = Some(manual_entry.into());
        self
    }
    /// List of data features that you would like to retrieve upon account creation.
    pub fn prefetch(mut self, prefetch: impl Into<Vec<stripe_shared::Prefetch>>) -> Self {
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

/// Type of account holder to collect accounts for.
pub type CreateFinancialConnectionsSessionAccountHolderType =
    stripe_shared::BankConnectionsResourceAccountholderType;
/// Restricts the Session to subcategories of accounts that can be linked.
/// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
pub type CreateFinancialConnectionsSessionFiltersAccountSubcategories =
    stripe_shared::AccountSubcategories;
/// Whether the session should require payment method support and successful account number retrieval before completion.
pub type CreateFinancialConnectionsSessionFiltersRequirePaymentMethodSupport =
    stripe_shared::RequirePaymentMethodSupport;
/// How manual entry should be handled.
pub type CreateFinancialConnectionsSessionManualEntryMode = stripe_shared::Mode;
/// List of data features that you would like to request access to.
///
/// Possible values are `balances`, `transactions`, `ownership`, and `payment_method`.
pub type CreateFinancialConnectionsSessionPermissions = stripe_shared::Permissions;
/// List of data features that you would like to retrieve upon account creation.
pub type CreateFinancialConnectionsSessionPrefetch = stripe_shared::Prefetch;
