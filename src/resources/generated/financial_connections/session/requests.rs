use crate::{Client, Response};

impl crate::financial_connections::session::Session {
    /// To launch the Financial Connections authorization flow, create a `Session`.
    ///
    /// The session’s `client_secret` can be used to launch the flow using Stripe.js.
    pub fn create(
        client: &Client,
        params: CreateSession,
    ) -> Response<crate::financial_connections::session::Session> {
        client.send_form("/financial_connections/sessions", params, http_types::Method::Post)
    }
    /// Retrieves the details of a Financial Connections `Session`.
    pub fn retrieve(
        client: &Client,
        session: &str,
        params: RetrieveSession,
    ) -> Response<crate::financial_connections::session::Session> {
        client.get_query(
            &format!("/financial_connections/sessions/{session}", session = session),
            params,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSession<'a> {
    /// The account holder to link accounts for.
    pub account_holder: CreateSessionAccountHolder<'a>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Filters to restrict the kinds of accounts to collect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<CreateSessionFilters<'a>>,
    /// List of data features that you would like to request access to.
    ///
    /// Possible values are `balances`, `transactions`, `ownership`, and `payment_method`.
    pub permissions: &'a [CreateSessionPermissions],
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> CreateSession<'a> {
    pub fn new(
        account_holder: CreateSessionAccountHolder<'a>,
        permissions: &'a [CreateSessionPermissions],
    ) -> Self {
        Self {
            account_holder,
            expand: Default::default(),
            filters: Default::default(),
            permissions,
            return_url: Default::default(),
        }
    }
}
/// The account holder to link accounts for.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionAccountHolder<'a> {
    /// The ID of the Stripe account whose accounts will be retrieved.
    ///
    /// Should only be present if `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// The ID of the Stripe customer whose accounts will be retrieved.
    ///
    /// Should only be present if `type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Type of account holder to collect accounts for.
    #[serde(rename = "type")]
    pub type_: CreateSessionAccountHolderType,
}
impl<'a> CreateSessionAccountHolder<'a> {
    pub fn new(type_: CreateSessionAccountHolderType) -> Self {
        Self { account: Default::default(), customer: Default::default(), type_ }
    }
}
/// Type of account holder to collect accounts for.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionAccountHolderType {
    Account,
    Customer,
}

impl CreateSessionAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Customer => "customer",
        }
    }
}

impl AsRef<str> for CreateSessionAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Filters to restrict the kinds of accounts to collect.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionFilters<'a> {
    /// List of countries from which to collect accounts.
    pub countries: &'a [&'a str],
}
impl<'a> CreateSessionFilters<'a> {
    pub fn new(countries: &'a [&'a str]) -> Self {
        Self { countries }
    }
}
/// List of data features that you would like to request access to.
///
/// Possible values are `balances`, `transactions`, `ownership`, and `payment_method`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateSessionPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for CreateSessionPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
