// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{FinancialConnectionsAccountId};
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{BankConnectionsResourceAccountholder, FinancialConnectionsAccountOwnership};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BankConnectionsResourceLinkedAccount".
///
/// For more details see <https://stripe.com/docs/api/financial_connections/accounts/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialConnectionsAccount {
    /// Unique identifier for the object.
    pub id: FinancialConnectionsAccountId,

    /// The account holder that this account belongs to.
    pub account_holder: Option<BankConnectionsResourceAccountholder>,

    /// The most recent information about the account's balance.
    pub balance: Option<BankConnectionsResourceBalance>,

    /// The state of the most recent attempt to refresh the account balance.
    pub balance_refresh: Option<BankConnectionsResourceBalanceRefresh>,

    /// The type of the account.
    ///
    /// Account category is further divided in `subcategory`.
    pub category: FinancialConnectionsAccountCategory,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// A human-readable name that has been assigned to this account, either by the account holder or by the institution.
    pub display_name: Option<String>,

    /// The name of the institution that holds this account.
    pub institution_name: String,

    /// The last 4 digits of the account number.
    ///
    /// If present, this will be 4 numeric characters.
    pub last4: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The most recent information about the account's owners.
    pub ownership: Option<Expandable<FinancialConnectionsAccountOwnership>>,

    /// The state of the most recent attempt to refresh the account owners.
    pub ownership_refresh: Option<BankConnectionsResourceOwnershipRefresh>,

    /// The list of permissions granted by this account.
    pub permissions: Option<Vec<FinancialConnectionsAccountPermissions>>,

    /// The status of the link to the account.
    pub status: FinancialConnectionsAccountStatus,

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
    pub subcategory: FinancialConnectionsAccountSubcategory,

    /// The list of data refresh subscriptions requested on this account.
    pub subscriptions: Option<Vec<FinancialConnectionsAccountSubscriptions>>,

    /// The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
    pub supported_payment_method_types: Vec<FinancialConnectionsAccountSupportedPaymentMethodTypes>,

    /// The state of the most recent attempt to refresh the account transactions.
    pub transaction_refresh: Option<BankConnectionsResourceTransactionRefresh>,
}

impl Object for FinancialConnectionsAccount {
    type Id = FinancialConnectionsAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "financial_connections.account"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankConnectionsResourceBalance {

    /// The time that the external institution calculated this balance.
    ///
    /// Measured in seconds since the Unix epoch.
    pub as_of: Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash: Option<BankConnectionsResourceBalanceApiResourceCashBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<BankConnectionsResourceBalanceApiResourceCreditBalance>,

    /// The balances owed to (or by) the account holder, before subtracting any outbound pending transactions or adding any inbound pending transactions.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    ///
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub current: i64,

    /// The `type` of the balance.
    ///
    /// An additional hash is included on the balance with a name matching this value.
    #[serde(rename = "type")]
    pub type_: BankConnectionsResourceBalanceType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankConnectionsResourceBalanceApiResourceCashBalance {

    /// The funds available to the account holder.
    ///
    /// Typically this is the current balance after subtracting any outbound pending transactions and adding any inbound pending transactions.  Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.  Each value is a integer amount.
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub available: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalance {

    /// The credit that has been used by the account holder.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    ///
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub used: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankConnectionsResourceBalanceRefresh {

    /// The time at which the last refresh attempt was initiated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub last_attempted_at: Timestamp,

    /// Time at which the next balance refresh can be initiated.
    ///
    /// This value will be `null` when `status` is `pending`.
    /// Measured in seconds since the Unix epoch.
    pub next_refresh_available_at: Option<Timestamp>,

    /// The status of the last refresh attempt.
    pub status: BankConnectionsResourceBalanceRefreshStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankConnectionsResourceOwnershipRefresh {

    /// The time at which the last refresh attempt was initiated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub last_attempted_at: Timestamp,

    /// Time at which the next ownership refresh can be initiated.
    ///
    /// This value will be `null` when `status` is `pending`.
    /// Measured in seconds since the Unix epoch.
    pub next_refresh_available_at: Option<Timestamp>,

    /// The status of the last refresh attempt.
    pub status: BankConnectionsResourceOwnershipRefreshStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankConnectionsResourceTransactionRefresh {

    /// Unique identifier for the object.
    pub id: String,

    /// The time at which the last refresh attempt was initiated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub last_attempted_at: Timestamp,

    /// Time at which the next transaction refresh can be initiated.
    ///
    /// This value will be `null` when `status` is `pending`.
    /// Measured in seconds since the Unix epoch.
    pub next_refresh_available_at: Option<Timestamp>,

    /// The status of the last refresh attempt.
    pub status: BankConnectionsResourceTransactionRefreshStatus,
}

/// An enum representing the possible values of an `BankConnectionsResourceBalanceRefresh`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceBalanceRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

impl BankConnectionsResourceBalanceRefreshStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            BankConnectionsResourceBalanceRefreshStatus::Failed => "failed",
            BankConnectionsResourceBalanceRefreshStatus::Pending => "pending",
            BankConnectionsResourceBalanceRefreshStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for BankConnectionsResourceBalanceRefreshStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceBalanceRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BankConnectionsResourceBalanceRefreshStatus {
    fn default() -> Self {
        Self::Failed
    }
}

/// An enum representing the possible values of an `BankConnectionsResourceBalance`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceBalanceType {
    Cash,
    Credit,
}

impl BankConnectionsResourceBalanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            BankConnectionsResourceBalanceType::Cash => "cash",
            BankConnectionsResourceBalanceType::Credit => "credit",
        }
    }
}

impl AsRef<str> for BankConnectionsResourceBalanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BankConnectionsResourceBalanceType {
    fn default() -> Self {
        Self::Cash
    }
}

/// An enum representing the possible values of an `BankConnectionsResourceOwnershipRefresh`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceOwnershipRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

impl BankConnectionsResourceOwnershipRefreshStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            BankConnectionsResourceOwnershipRefreshStatus::Failed => "failed",
            BankConnectionsResourceOwnershipRefreshStatus::Pending => "pending",
            BankConnectionsResourceOwnershipRefreshStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for BankConnectionsResourceOwnershipRefreshStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceOwnershipRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BankConnectionsResourceOwnershipRefreshStatus {
    fn default() -> Self {
        Self::Failed
    }
}

/// An enum representing the possible values of an `BankConnectionsResourceTransactionRefresh`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceTransactionRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

impl BankConnectionsResourceTransactionRefreshStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            BankConnectionsResourceTransactionRefreshStatus::Failed => "failed",
            BankConnectionsResourceTransactionRefreshStatus::Pending => "pending",
            BankConnectionsResourceTransactionRefreshStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for BankConnectionsResourceTransactionRefreshStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceTransactionRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BankConnectionsResourceTransactionRefreshStatus {
    fn default() -> Self {
        Self::Failed
    }
}

/// An enum representing the possible values of an `FinancialConnectionsAccount`'s `category` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountCategory {
    Cash,
    Credit,
    Investment,
    Other,
}

impl FinancialConnectionsAccountCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancialConnectionsAccountCategory::Cash => "cash",
            FinancialConnectionsAccountCategory::Credit => "credit",
            FinancialConnectionsAccountCategory::Investment => "investment",
            FinancialConnectionsAccountCategory::Other => "other",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FinancialConnectionsAccountCategory {
    fn default() -> Self {
        Self::Cash
    }
}

/// An enum representing the possible values of an `FinancialConnectionsAccount`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl FinancialConnectionsAccountPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancialConnectionsAccountPermissions::Balances => "balances",
            FinancialConnectionsAccountPermissions::Ownership => "ownership",
            FinancialConnectionsAccountPermissions::PaymentMethod => "payment_method",
            FinancialConnectionsAccountPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FinancialConnectionsAccountPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `FinancialConnectionsAccount`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountStatus {
    Active,
    Disconnected,
    Inactive,
}

impl FinancialConnectionsAccountStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancialConnectionsAccountStatus::Active => "active",
            FinancialConnectionsAccountStatus::Disconnected => "disconnected",
            FinancialConnectionsAccountStatus::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FinancialConnectionsAccountStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `FinancialConnectionsAccount`'s `subcategory` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountSubcategory {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Other,
    Savings,
}

impl FinancialConnectionsAccountSubcategory {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancialConnectionsAccountSubcategory::Checking => "checking",
            FinancialConnectionsAccountSubcategory::CreditCard => "credit_card",
            FinancialConnectionsAccountSubcategory::LineOfCredit => "line_of_credit",
            FinancialConnectionsAccountSubcategory::Mortgage => "mortgage",
            FinancialConnectionsAccountSubcategory::Other => "other",
            FinancialConnectionsAccountSubcategory::Savings => "savings",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountSubcategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FinancialConnectionsAccountSubcategory {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `FinancialConnectionsAccount`'s `subscriptions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountSubscriptions {
    Transactions,
}

impl FinancialConnectionsAccountSubscriptions {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancialConnectionsAccountSubscriptions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountSubscriptions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountSubscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FinancialConnectionsAccountSubscriptions {
    fn default() -> Self {
        Self::Transactions
    }
}

/// An enum representing the possible values of an `FinancialConnectionsAccount`'s `supported_payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountSupportedPaymentMethodTypes {
    Link,
    UsBankAccount,
}

impl FinancialConnectionsAccountSupportedPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancialConnectionsAccountSupportedPaymentMethodTypes::Link => "link",
            FinancialConnectionsAccountSupportedPaymentMethodTypes::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn default() -> Self {
        Self::Link
    }
}
