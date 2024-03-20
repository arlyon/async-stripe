#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFinancialConnectionsSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFinancialConnectionsSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveFinancialConnectionsSession<'a> {
    /// Retrieves the details of a Financial Connections `Session`
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_misc::FinancialConnectionsSessionId,
    ) -> stripe::Response<stripe_misc::FinancialConnectionsSession> {
        client.get_query(&format!("/financial_connections/sessions/{session}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialConnectionsSession<'a> {
    /// The account holder to link accounts for.
    pub account_holder: CreateFinancialConnectionsSessionAccountHolder<'a>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Filters to restrict the kinds of accounts to collect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<CreateFinancialConnectionsSessionFilters<'a>>,
    /// List of data features that you would like to request access to.
    ///
    /// Possible values are `balances`, `transactions`, `ownership`, and `payment_method`.
    pub permissions: &'a [stripe_misc::FinancialConnectionsSessionPermissions],
    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<&'a [stripe_misc::FinancialConnectionsSessionPrefetch]>,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> CreateFinancialConnectionsSession<'a> {
    pub fn new(
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFinancialConnectionsSessionAccountHolderType::*;
        match s {
            "account" => Ok(Account),
            "customer" => Ok(Customer),
            _ => Err(()),
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
impl<'a> CreateFinancialConnectionsSession<'a> {
    /// To launch the Financial Connections authorization flow, create a `Session`.
    /// The session’s `client_secret` can be used to launch the flow using Stripe.js.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_misc::FinancialConnectionsSession> {
        client.send_form("/financial_connections/sessions", self, http_types::Method::Post)
    }
}
