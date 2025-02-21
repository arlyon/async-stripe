// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{FinancialConnectionsSessionId};
use crate::params::{Expand, List, Object};
use crate::resources::{BankConnectionsResourceAccountholder, FinancialConnectionsAccount};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BankConnectionsResourceLinkAccountSession".
///
/// For more details see <https://stripe.com/docs/api/financial_connections/sessions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialConnectionsSession {
    /// Unique identifier for the object.
    pub id: FinancialConnectionsSessionId,

    /// The account holder for whom accounts are collected in this session.
    pub account_holder: Option<BankConnectionsResourceAccountholder>,

    /// The accounts that were collected as part of this Session.
    pub accounts: List<FinancialConnectionsAccount>,

    /// A value that will be passed to the client to launch the authentication flow.
    pub client_secret: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<BankConnectionsResourceLinkAccountSessionFilters>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Permissions requested for accounts collected during this session.
    pub permissions: Vec<FinancialConnectionsSessionPermissions>,

    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<FinancialConnectionsSessionPrefetch>>,

    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

impl FinancialConnectionsSession {

    /// To launch the Financial Connections authorization flow, create a `Session`.
    ///
    /// The session’s `client_secret` can be used to launch the flow using Stripe.js.
    pub fn create(client: &Client, params: CreateFinancialConnectionsSession<'_>) -> Response<FinancialConnectionsSession> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/financial_connections/sessions", &params)
    }
}

impl Object for FinancialConnectionsSession {
    type Id = FinancialConnectionsSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "financial_connections.session"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankConnectionsResourceLinkAccountSessionFilters {

    /// Restricts the Session to subcategories of accounts that can be linked.
    ///
    /// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
    pub account_subcategories: Option<Vec<BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories>>,

    /// List of countries from which to filter accounts.
    pub countries: Option<Vec<String>>,
}

/// The parameters for `FinancialConnectionsSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateFinancialConnectionsSession<'a> {

    /// The account holder to link accounts for.
    pub account_holder: CreateFinancialConnectionsSessionAccountHolder,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Filters to restrict the kinds of accounts to collect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<CreateFinancialConnectionsSessionFilters>,

    /// List of data features that you would like to request access to.
    ///
    /// Possible values are `balances`, `transactions`, `ownership`, and `payment_method`.
    pub permissions: Vec<CreateFinancialConnectionsSessionPermissions>,

    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<CreateFinancialConnectionsSessionPrefetch>>,

    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}

impl<'a> CreateFinancialConnectionsSession<'a> {
    pub fn new(account_holder: CreateFinancialConnectionsSessionAccountHolder, permissions: Vec<CreateFinancialConnectionsSessionPermissions>) -> Self {
        CreateFinancialConnectionsSession {
            account_holder,
            expand: Default::default(),
            filters: Default::default(),
            permissions,
            prefetch: Default::default(),
            return_url: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateFinancialConnectionsSessionAccountHolder {

    /// The ID of the Stripe account whose accounts will be retrieved.
    ///
    /// Should only be present if `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// The ID of the Stripe customer whose accounts will be retrieved.
    ///
    /// Should only be present if `type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Type of account holder to collect accounts for.
    #[serde(rename = "type")]
    pub type_: CreateFinancialConnectionsSessionAccountHolderType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateFinancialConnectionsSessionFilters {

    /// Restricts the Session to subcategories of accounts that can be linked.
    ///
    /// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_subcategories: Option<Vec<CreateFinancialConnectionsSessionFiltersAccountSubcategories>>,

    /// List of countries from which to collect accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
}

/// An enum representing the possible values of an `BankConnectionsResourceLinkAccountSessionFilters`'s `account_subcategories` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Savings,
}

impl BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    pub fn as_str(self) -> &'static str {
        match self {
            BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::Checking => "checking",
            BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::CreditCard => "credit_card",
            BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::LineOfCredit => "line_of_credit",
            BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::Mortgage => "mortgage",
            BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::Savings => "savings",
        }
    }
}

impl AsRef<str> for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `CreateFinancialConnectionsSessionAccountHolder`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateFinancialConnectionsSessionAccountHolderType {
    Account,
    Customer,
}

impl CreateFinancialConnectionsSessionAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateFinancialConnectionsSessionAccountHolderType::Account => "account",
            CreateFinancialConnectionsSessionAccountHolderType::Customer => "customer",
        }
    }
}

impl AsRef<str> for CreateFinancialConnectionsSessionAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateFinancialConnectionsSessionAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateFinancialConnectionsSessionAccountHolderType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreateFinancialConnectionsSessionFilters`'s `account_subcategories` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Savings,
}

impl CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateFinancialConnectionsSessionFiltersAccountSubcategories::Checking => "checking",
            CreateFinancialConnectionsSessionFiltersAccountSubcategories::CreditCard => "credit_card",
            CreateFinancialConnectionsSessionFiltersAccountSubcategories::LineOfCredit => "line_of_credit",
            CreateFinancialConnectionsSessionFiltersAccountSubcategories::Mortgage => "mortgage",
            CreateFinancialConnectionsSessionFiltersAccountSubcategories::Savings => "savings",
        }
    }
}

impl AsRef<str> for CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateFinancialConnectionsSessionFiltersAccountSubcategories {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `CreateFinancialConnectionsSession`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateFinancialConnectionsSessionPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateFinancialConnectionsSessionPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateFinancialConnectionsSessionPermissions::Balances => "balances",
            CreateFinancialConnectionsSessionPermissions::Ownership => "ownership",
            CreateFinancialConnectionsSessionPermissions::PaymentMethod => "payment_method",
            CreateFinancialConnectionsSessionPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for CreateFinancialConnectionsSessionPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateFinancialConnectionsSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateFinancialConnectionsSessionPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `CreateFinancialConnectionsSession`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateFinancialConnectionsSessionPrefetch {
    Balances,
    Ownership,
    Transactions,
}

impl CreateFinancialConnectionsSessionPrefetch {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateFinancialConnectionsSessionPrefetch::Balances => "balances",
            CreateFinancialConnectionsSessionPrefetch::Ownership => "ownership",
            CreateFinancialConnectionsSessionPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for CreateFinancialConnectionsSessionPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateFinancialConnectionsSessionPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateFinancialConnectionsSessionPrefetch {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `FinancialConnectionsSession`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsSessionPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl FinancialConnectionsSessionPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancialConnectionsSessionPermissions::Balances => "balances",
            FinancialConnectionsSessionPermissions::Ownership => "ownership",
            FinancialConnectionsSessionPermissions::PaymentMethod => "payment_method",
            FinancialConnectionsSessionPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for FinancialConnectionsSessionPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FinancialConnectionsSessionPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `FinancialConnectionsSession`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsSessionPrefetch {
    Balances,
    Ownership,
    Transactions,
}

impl FinancialConnectionsSessionPrefetch {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancialConnectionsSessionPrefetch::Balances => "balances",
            FinancialConnectionsSessionPrefetch::Ownership => "ownership",
            FinancialConnectionsSessionPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for FinancialConnectionsSessionPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsSessionPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FinancialConnectionsSessionPrefetch {
    fn default() -> Self {
        Self::Balances
    }
}
