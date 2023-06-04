/// A Financial Connections Account represents an account that exists outside of Stripe, to which you have been granted some degree of access.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Account {
    /// The account holder that this account belongs to.
    pub account_holder:
        Option<crate::financial_connections::account::account_holder::AccountHolder>,
    /// The most recent information about the account's balance.
    pub balance: Option<crate::financial_connections::account::balance::Balance>,
    /// The state of the most recent attempt to refresh the account balance.
    pub balance_refresh:
        Option<crate::financial_connections::account::balance_refresh::BalanceRefresh>,
    /// The type of the account.
    ///
    /// Account category is further divided in `subcategory`.
    pub category: AccountCategory,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// A human-readable name that has been assigned to this account, either by the account holder or by the institution.
    pub display_name: Option<String>,
    /// Unique identifier for the object.
    pub id: crate::financial_connections::account::FinancialConnectionsAccountId,
    /// The name of the institution that holds this account.
    pub institution_name: String,
    /// The last 4 digits of the account number.
    ///
    /// If present, this will be 4 numeric characters.
    pub last4: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: AccountObject,
    /// The most recent information about the account's owners.
    pub ownership: Option<
        crate::Expandable<crate::financial_connections::account_ownership::AccountOwnership>,
    >,
    /// The state of the most recent attempt to refresh the account owners.
    pub ownership_refresh:
        Option<crate::financial_connections::account::ownership_refresh::OwnershipRefresh>,
    /// The list of permissions granted by this account.
    pub permissions: Option<Vec<AccountPermissions>>,
    /// The status of the link to the account.
    pub status: AccountStatus,
    /// If `category` is `cash`, one of:
    ///
    ///  - `checking`
    ///  - `savings`
    ///  - `other`
    ///
    /// If `category` is `credit`, one of:
    ///
    ///  - `mortgage`
    ///  - `line_of_credit`
    ///  - `credit_card`
    ///  - `other`
    ///
    /// If `category` is `investment` or `other`, this will be `other`.
    pub subcategory: AccountSubcategory,
    /// The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
    pub supported_payment_method_types: Vec<AccountSupportedPaymentMethodTypes>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Account {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of the account.
///
/// Account category is further divided in `subcategory`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AccountCategory {
    Cash,
    Credit,
    Investment,
    Other,
}

impl AccountCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::Credit => "credit",
            Self::Investment => "investment",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for AccountCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AccountObject {
    #[serde(rename = "financial_connections.account")]
    FinancialConnectionsAccount,
}

impl AccountObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialConnectionsAccount => "financial_connections.account",
        }
    }
}

impl AsRef<str> for AccountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of permissions granted by this account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl AccountPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for AccountPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The status of the link to the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AccountStatus {
    Active,
    Disconnected,
    Inactive,
}

impl AccountStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disconnected => "disconnected",
            Self::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for AccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If `category` is `cash`, one of:
///
///  - `checking`
///  - `savings`
///  - `other`
///
/// If `category` is `credit`, one of:
///
///  - `mortgage`
///  - `line_of_credit`
///  - `credit_card`
///  - `other`
///
/// If `category` is `investment` or `other`, this will be `other`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AccountSubcategory {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Other,
    Savings,
}

impl AccountSubcategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::CreditCard => "credit_card",
            Self::LineOfCredit => "line_of_credit",
            Self::Mortgage => "mortgage",
            Self::Other => "other",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for AccountSubcategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AccountSupportedPaymentMethodTypes {
    Link,
    UsBankAccount,
}

impl AccountSupportedPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Link => "link",
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for AccountSupportedPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for Account {
    type Id = crate::financial_connections::account::FinancialConnectionsAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(FinancialConnectionsAccountId);
pub mod account_holder;
pub mod requests;
pub use account_holder::AccountHolder;
pub mod balance;
pub use balance::Balance;
pub mod balance_refresh;
pub use balance_refresh::BalanceRefresh;
pub mod ownership_refresh;
pub use ownership_refresh::OwnershipRefresh;
