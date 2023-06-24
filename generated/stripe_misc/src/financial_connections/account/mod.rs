/// A Financial Connections Account represents an account that exists outside of Stripe, to which you have been granted some degree of access.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Account {
    /// The account holder that this account belongs to.
    pub account_holder:
        Option<stripe_misc::financial_connections::account::account_holder::AccountHolder>,
    /// The most recent information about the account's balance.
    pub balance: Option<stripe_misc::financial_connections::account::balance::Balance>,
    /// The state of the most recent attempt to refresh the account balance.
    pub balance_refresh:
        Option<stripe_misc::financial_connections::account::balance_refresh::BalanceRefresh>,
    /// The type of the account.
    ///
    /// Account category is further divided in `subcategory`.
    pub category: AccountCategory,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// A human-readable name that has been assigned to this account, either by the account holder or by the institution.
    pub display_name: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::financial_connections::account::FinancialConnectionsAccountId,
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
        stripe_types::Expandable<
            stripe_misc::financial_connections::account_ownership::AccountOwnership,
        >,
    >,
    /// The state of the most recent attempt to refresh the account owners.
    pub ownership_refresh:
        Option<stripe_misc::financial_connections::account::ownership_refresh::OwnershipRefresh>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for AccountCategory {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cash" => Ok(Self::Cash),
            "credit" => Ok(Self::Credit),
            "investment" => Ok(Self::Investment),
            "other" => Ok(Self::Other),

            _ => Err(()),
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
impl serde::Serialize for AccountCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountCategory"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountCategory {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AccountCategory> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountCategory::from_str(s)?);
        Ok(())
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountObject {
    FinancialConnectionsAccount,
}

impl AccountObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialConnectionsAccount => "financial_connections.account",
        }
    }
}

impl std::str::FromStr for AccountObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "financial_connections.account" => Ok(Self::FinancialConnectionsAccount),

            _ => Err(()),
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
impl serde::Serialize for AccountObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for AccountObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AccountObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountObject::from_str(s)?);
        Ok(())
    }
}
/// The list of permissions granted by this account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for AccountPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "balances" => Ok(Self::Balances),
            "ownership" => Ok(Self::Ownership),
            "payment_method" => Ok(Self::PaymentMethod),
            "transactions" => Ok(Self::Transactions),

            _ => Err(()),
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
impl serde::Serialize for AccountPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountPermissions"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AccountPermissions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountPermissions::from_str(s)?);
        Ok(())
    }
}
/// The status of the link to the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for AccountStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "disconnected" => Ok(Self::Disconnected),
            "inactive" => Ok(Self::Inactive),

            _ => Err(()),
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
impl serde::Serialize for AccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for AccountStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AccountStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountStatus::from_str(s)?);
        Ok(())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for AccountSubcategory {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "checking" => Ok(Self::Checking),
            "credit_card" => Ok(Self::CreditCard),
            "line_of_credit" => Ok(Self::LineOfCredit),
            "mortgage" => Ok(Self::Mortgage),
            "other" => Ok(Self::Other),
            "savings" => Ok(Self::Savings),

            _ => Err(()),
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
impl serde::Serialize for AccountSubcategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountSubcategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountSubcategory"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountSubcategory {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AccountSubcategory> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountSubcategory::from_str(s)?);
        Ok(())
    }
}
/// The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for AccountSupportedPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "link" => Ok(Self::Link),
            "us_bank_account" => Ok(Self::UsBankAccount),

            _ => Err(()),
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
impl serde::Serialize for AccountSupportedPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountSupportedPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountSupportedPaymentMethodTypes")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountSupportedPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AccountSupportedPaymentMethodTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountSupportedPaymentMethodTypes::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for Account {
    type Id = stripe_misc::financial_connections::account::FinancialConnectionsAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FinancialConnectionsAccountId);
pub mod account_holder;
pub mod requests;
pub use account_holder::AccountHolder;
pub mod balance;
pub use balance::Balance;
pub mod balance_refresh;
pub use balance_refresh::BalanceRefresh;
pub mod ownership_refresh;
pub use ownership_refresh::OwnershipRefresh;
