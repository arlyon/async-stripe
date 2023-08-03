/// A Financial Connections Account represents an account that exists outside of Stripe, to which you have been granted some degree of access.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceLinkedAccount {
    /// The account holder that this account belongs to.
    pub account_holder: Option<stripe_misc::BankConnectionsResourceAccountholder>,
    /// The most recent information about the account's balance.
    pub balance: Option<stripe_misc::BankConnectionsResourceBalance>,
    /// The state of the most recent attempt to refresh the account balance.
    pub balance_refresh: Option<stripe_misc::BankConnectionsResourceBalanceRefresh>,
    /// The type of the account.
    ///
    /// Account category is further divided in `subcategory`.
    pub category: BankConnectionsResourceLinkedAccountCategory,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// A human-readable name that has been assigned to this account, either by the account holder or by the institution.
    pub display_name: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::bank_connections_resource_linked_account::FinancialConnectionsAccountId,
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
    pub object: BankConnectionsResourceLinkedAccountObject,
    /// The most recent information about the account's owners.
    pub ownership: Option<stripe_types::Expandable<stripe_misc::BankConnectionsResourceOwnership>>,
    /// The state of the most recent attempt to refresh the account owners.
    pub ownership_refresh: Option<stripe_misc::BankConnectionsResourceOwnershipRefresh>,
    /// The list of permissions granted by this account.
    pub permissions: Option<Vec<BankConnectionsResourceLinkedAccountPermissions>>,
    /// The status of the link to the account.
    pub status: BankConnectionsResourceLinkedAccountStatus,
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
    pub subcategory: BankConnectionsResourceLinkedAccountSubcategory,
    /// The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
    pub supported_payment_method_types: Vec<BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes>,
}
/// The type of the account.
///
/// Account category is further divided in `subcategory`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceLinkedAccountCategory {
    Cash,
    Credit,
    Investment,
    Other,
}

impl BankConnectionsResourceLinkedAccountCategory {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceLinkedAccountCategory::*;
        match self {
            Cash => "cash",
            Credit => "credit",
            Investment => "investment",
            Other => "other",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceLinkedAccountCategory {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkedAccountCategory::*;
        match s {
            "cash" => Ok(Cash),
            "credit" => Ok(Credit),
            "investment" => Ok(Investment),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankConnectionsResourceLinkedAccountCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceLinkedAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceLinkedAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceLinkedAccountCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceLinkedAccountCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceLinkedAccountCategory"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceLinkedAccountObject {
    FinancialConnectionsAccount,
}

impl BankConnectionsResourceLinkedAccountObject {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceLinkedAccountObject::*;
        match self {
            FinancialConnectionsAccount => "financial_connections.account",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceLinkedAccountObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkedAccountObject::*;
        match s {
            "financial_connections.account" => Ok(FinancialConnectionsAccount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankConnectionsResourceLinkedAccountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceLinkedAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceLinkedAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceLinkedAccountObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceLinkedAccountObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceLinkedAccountObject"))
    }
}
/// The list of permissions granted by this account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceLinkedAccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl BankConnectionsResourceLinkedAccountPermissions {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceLinkedAccountPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceLinkedAccountPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkedAccountPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankConnectionsResourceLinkedAccountPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceLinkedAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceLinkedAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceLinkedAccountPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceLinkedAccountPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceLinkedAccountPermissions"))
    }
}
/// The status of the link to the account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceLinkedAccountStatus {
    Active,
    Disconnected,
    Inactive,
}

impl BankConnectionsResourceLinkedAccountStatus {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceLinkedAccountStatus::*;
        match self {
            Active => "active",
            Disconnected => "disconnected",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceLinkedAccountStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkedAccountStatus::*;
        match s {
            "active" => Ok(Active),
            "disconnected" => Ok(Disconnected),
            "inactive" => Ok(Inactive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankConnectionsResourceLinkedAccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceLinkedAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceLinkedAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceLinkedAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceLinkedAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceLinkedAccountStatus"))
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
pub enum BankConnectionsResourceLinkedAccountSubcategory {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Other,
    Savings,
}

impl BankConnectionsResourceLinkedAccountSubcategory {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceLinkedAccountSubcategory::*;
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

impl std::str::FromStr for BankConnectionsResourceLinkedAccountSubcategory {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkedAccountSubcategory::*;
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

impl AsRef<str> for BankConnectionsResourceLinkedAccountSubcategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceLinkedAccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceLinkedAccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceLinkedAccountSubcategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceLinkedAccountSubcategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceLinkedAccountSubcategory"))
    }
}
/// The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes {
    Link,
    UsBankAccount,
}

impl BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes::*;
        match self {
            Link => "link",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes::*;
        match s {
            "link" => Ok(Link),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceLinkedAccountSupportedPaymentMethodTypes"))
    }
}
impl stripe_types::Object for BankConnectionsResourceLinkedAccount {
    type Id = stripe_misc::bank_connections_resource_linked_account::FinancialConnectionsAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FinancialConnectionsAccountId);
pub mod requests;
