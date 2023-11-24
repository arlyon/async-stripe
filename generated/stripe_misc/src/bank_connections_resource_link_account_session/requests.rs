#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBankConnectionsResourceLinkAccountSession<'a> {
    /// The account holder to link accounts for.
    pub account_holder: CreateBankConnectionsResourceLinkAccountSessionAccountHolder<'a>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Filters to restrict the kinds of accounts to collect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<CreateBankConnectionsResourceLinkAccountSessionFilters<'a>>,
    /// List of data features that you would like to request access to.
    ///
    /// Possible values are `balances`, `transactions`, `ownership`, and `payment_method`.
    pub permissions: &'a [CreateBankConnectionsResourceLinkAccountSessionPermissions],
    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<&'a [CreateBankConnectionsResourceLinkAccountSessionPrefetch]>,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> CreateBankConnectionsResourceLinkAccountSession<'a> {
    pub fn new(
        account_holder: CreateBankConnectionsResourceLinkAccountSessionAccountHolder<'a>,
        permissions: &'a [CreateBankConnectionsResourceLinkAccountSessionPermissions],
    ) -> Self {
        Self {
            account_holder,
            expand: Default::default(),
            filters: Default::default(),
            permissions,
            prefetch: Default::default(),
            return_url: Default::default(),
        }
    }
}
/// The account holder to link accounts for.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBankConnectionsResourceLinkAccountSessionAccountHolder<'a> {
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
    pub type_: CreateBankConnectionsResourceLinkAccountSessionAccountHolderType,
}
impl<'a> CreateBankConnectionsResourceLinkAccountSessionAccountHolder<'a> {
    pub fn new(type_: CreateBankConnectionsResourceLinkAccountSessionAccountHolderType) -> Self {
        Self { account: Default::default(), customer: Default::default(), type_ }
    }
}
/// Type of account holder to collect accounts for.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBankConnectionsResourceLinkAccountSessionAccountHolderType {
    Account,
    Customer,
}

impl CreateBankConnectionsResourceLinkAccountSessionAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreateBankConnectionsResourceLinkAccountSessionAccountHolderType::*;
        match self {
            Account => "account",
            Customer => "customer",
        }
    }
}

impl std::str::FromStr for CreateBankConnectionsResourceLinkAccountSessionAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBankConnectionsResourceLinkAccountSessionAccountHolderType::*;
        match s {
            "account" => Ok(Account),
            "customer" => Ok(Customer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateBankConnectionsResourceLinkAccountSessionAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateBankConnectionsResourceLinkAccountSessionAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBankConnectionsResourceLinkAccountSessionAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBankConnectionsResourceLinkAccountSessionAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Filters to restrict the kinds of accounts to collect.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBankConnectionsResourceLinkAccountSessionFilters<'a> {
    /// List of countries from which to collect accounts.
    pub countries: &'a [&'a str],
}
impl<'a> CreateBankConnectionsResourceLinkAccountSessionFilters<'a> {
    pub fn new(countries: &'a [&'a str]) -> Self {
        Self { countries }
    }
}
/// List of data features that you would like to request access to.
///
/// Possible values are `balances`, `transactions`, `ownership`, and `payment_method`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBankConnectionsResourceLinkAccountSessionPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateBankConnectionsResourceLinkAccountSessionPermissions {
    pub fn as_str(self) -> &'static str {
        use CreateBankConnectionsResourceLinkAccountSessionPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for CreateBankConnectionsResourceLinkAccountSessionPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBankConnectionsResourceLinkAccountSessionPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateBankConnectionsResourceLinkAccountSessionPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateBankConnectionsResourceLinkAccountSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBankConnectionsResourceLinkAccountSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBankConnectionsResourceLinkAccountSessionPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBankConnectionsResourceLinkAccountSessionPrefetch {
    Balances,
    Ownership,
}

impl CreateBankConnectionsResourceLinkAccountSessionPrefetch {
    pub fn as_str(self) -> &'static str {
        use CreateBankConnectionsResourceLinkAccountSessionPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
        }
    }
}

impl std::str::FromStr for CreateBankConnectionsResourceLinkAccountSessionPrefetch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBankConnectionsResourceLinkAccountSessionPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateBankConnectionsResourceLinkAccountSessionPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateBankConnectionsResourceLinkAccountSessionPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBankConnectionsResourceLinkAccountSessionPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBankConnectionsResourceLinkAccountSessionPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateBankConnectionsResourceLinkAccountSession<'a> {
    /// To launch the Financial Connections authorization flow, create a `Session`.
    ///
    /// The session’s `client_secret` can be used to launch the flow using Stripe.js.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_misc::BankConnectionsResourceLinkAccountSession> {
        client.send_form("/financial_connections/sessions", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveBankConnectionsResourceLinkAccountSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveBankConnectionsResourceLinkAccountSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveBankConnectionsResourceLinkAccountSession<'a> {
    /// Retrieves the details of a Financial Connections `Session`.
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &str,
    ) -> stripe::Response<stripe_misc::BankConnectionsResourceLinkAccountSession> {
        client.get_query(&format!("/financial_connections/sessions/{session}"), self)
    }
}
