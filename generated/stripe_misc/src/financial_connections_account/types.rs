/// A Financial Connections Account represents an account that exists outside of Stripe, to which you have been granted some degree of access.
///
/// For more details see <<https://stripe.com/docs/api/financial_connections/accounts/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FinancialConnectionsAccount {
    /// The account holder that this account belongs to.
    pub account_holder: Option<stripe_misc::BankConnectionsResourceAccountholder>,
    /// The most recent information about the account's balance.
    pub balance: Option<stripe_misc::BankConnectionsResourceBalance>,
    /// The state of the most recent attempt to refresh the account balance.
    pub balance_refresh: Option<stripe_misc::BankConnectionsResourceBalanceRefresh>,
    /// The type of the account. Account category is further divided in `subcategory`.
    pub category: FinancialConnectionsAccountCategory,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// A human-readable name that has been assigned to this account, either by the account holder or by the institution.
    pub display_name: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsAccountId,
    /// The name of the institution that holds this account.
    pub institution_name: String,
    /// The last 4 digits of the account number. If present, this will be 4 numeric characters.
    pub last4: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The most recent information about the account's owners.
    pub ownership:
        Option<stripe_types::Expandable<stripe_misc::FinancialConnectionsAccountOwnership>>,
    /// The state of the most recent attempt to refresh the account owners.
    pub ownership_refresh: Option<stripe_misc::BankConnectionsResourceOwnershipRefresh>,
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
    pub transaction_refresh: Option<stripe_misc::BankConnectionsResourceTransactionRefresh>,
}
/// The type of the account. Account category is further divided in `subcategory`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FinancialConnectionsAccountCategory {
    Cash,
    Credit,
    Investment,
    Other,
}
impl FinancialConnectionsAccountCategory {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsAccountCategory::*;
        match self {
            Cash => "cash",
            Credit => "credit",
            Investment => "investment",
            Other => "other",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountCategory {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountCategory::*;
        match s {
            "cash" => Ok(Cash),
            "credit" => Ok(Credit),
            "investment" => Ok(Investment),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FinancialConnectionsAccountCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsAccountCategory")
        })
    }
}
/// The list of permissions granted by this account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FinancialConnectionsAccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl FinancialConnectionsAccountPermissions {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsAccountPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FinancialConnectionsAccountPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsAccountPermissions")
        })
    }
}
/// The status of the link to the account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FinancialConnectionsAccountStatus {
    Active,
    Disconnected,
    Inactive,
}
impl FinancialConnectionsAccountStatus {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsAccountStatus::*;
        match self {
            Active => "active",
            Disconnected => "disconnected",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountStatus::*;
        match s {
            "active" => Ok(Active),
            "disconnected" => Ok(Disconnected),
            "inactive" => Ok(Inactive),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FinancialConnectionsAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsAccountStatus")
        })
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
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use FinancialConnectionsAccountSubcategory::*;
        match self {
            Checking => "checking",
            CreditCard => "credit_card",
            LineOfCredit => "line_of_credit",
            Mortgage => "mortgage",
            Other => "other",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountSubcategory {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountSubcategory::*;
        match s {
            "checking" => Ok(Checking),
            "credit_card" => Ok(CreditCard),
            "line_of_credit" => Ok(LineOfCredit),
            "mortgage" => Ok(Mortgage),
            "other" => Ok(Other),
            "savings" => Ok(Savings),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsAccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FinancialConnectionsAccountSubcategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountSubcategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsAccountSubcategory")
        })
    }
}
/// The list of data refresh subscriptions requested on this account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FinancialConnectionsAccountSubscriptions {
    Transactions,
}
impl FinancialConnectionsAccountSubscriptions {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsAccountSubscriptions::*;
        match self {
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountSubscriptions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountSubscriptions::*;
        match s {
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountSubscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsAccountSubscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FinancialConnectionsAccountSubscriptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountSubscriptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsAccountSubscriptions")
        })
    }
}
/// The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FinancialConnectionsAccountSupportedPaymentMethodTypes {
    Link,
    UsBankAccount,
}
impl FinancialConnectionsAccountSupportedPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsAccountSupportedPaymentMethodTypes::*;
        match self {
            Link => "link",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountSupportedPaymentMethodTypes::*;
        match s {
            "link" => Ok(Link),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for FinancialConnectionsAccountSupportedPaymentMethodTypes",
            )
        })
    }
}
impl stripe_types::Object for FinancialConnectionsAccount {
    type Id = stripe_misc::FinancialConnectionsAccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(FinancialConnectionsAccountId);
