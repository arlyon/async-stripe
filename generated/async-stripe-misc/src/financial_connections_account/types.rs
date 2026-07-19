/// A Financial Connections Account represents an account that exists outside of Stripe, to which you have been granted some degree of access.
///
/// For more details see <<https://stripe.com/docs/api/financial_connections/accounts/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FinancialConnectionsAccount {
    /// The account holder that this account belongs to.
    pub account_holder: Option<stripe_misc::BankConnectionsResourceAccountholder>,
    /// Details about the account numbers.
    pub account_numbers: Option<Vec<stripe_misc::BankConnectionsResourceAccountNumberDetails>>,
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
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
    /// The [PaymentMethod type](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
    pub supported_payment_method_types: Vec<FinancialConnectionsAccountSupportedPaymentMethodTypes>,
    /// The state of the most recent attempt to refresh the account transactions.
    pub transaction_refresh: Option<stripe_misc::BankConnectionsResourceTransactionRefresh>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FinancialConnectionsAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct FinancialConnectionsAccountBuilder {
    account_holder: Option<Option<stripe_misc::BankConnectionsResourceAccountholder>>,
    account_numbers: Option<Option<Vec<stripe_misc::BankConnectionsResourceAccountNumberDetails>>>,
    balance: Option<Option<stripe_misc::BankConnectionsResourceBalance>>,
    balance_refresh: Option<Option<stripe_misc::BankConnectionsResourceBalanceRefresh>>,
    category: Option<FinancialConnectionsAccountCategory>,
    created: Option<stripe_types::Timestamp>,
    display_name: Option<Option<String>>,
    id: Option<stripe_misc::FinancialConnectionsAccountId>,
    institution_name: Option<String>,
    last4: Option<Option<String>>,
    livemode: Option<bool>,
    ownership:
        Option<Option<stripe_types::Expandable<stripe_misc::FinancialConnectionsAccountOwnership>>>,
    ownership_refresh: Option<Option<stripe_misc::BankConnectionsResourceOwnershipRefresh>>,
    permissions: Option<Option<Vec<FinancialConnectionsAccountPermissions>>>,
    status: Option<FinancialConnectionsAccountStatus>,
    subcategory: Option<FinancialConnectionsAccountSubcategory>,
    subscriptions: Option<Option<Vec<FinancialConnectionsAccountSubscriptions>>>,
    supported_payment_method_types:
        Option<Vec<FinancialConnectionsAccountSupportedPaymentMethodTypes>>,
    transaction_refresh: Option<Option<stripe_misc::BankConnectionsResourceTransactionRefresh>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for FinancialConnectionsAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FinancialConnectionsAccount>,
        builder: FinancialConnectionsAccountBuilder,
    }

    impl Visitor for Place<FinancialConnectionsAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FinancialConnectionsAccountBuilder {
                    account_holder: Deserialize::default(),
                    account_numbers: Deserialize::default(),
                    balance: Deserialize::default(),
                    balance_refresh: Deserialize::default(),
                    category: Deserialize::default(),
                    created: Deserialize::default(),
                    display_name: Deserialize::default(),
                    id: Deserialize::default(),
                    institution_name: Deserialize::default(),
                    last4: Deserialize::default(),
                    livemode: Deserialize::default(),
                    ownership: Deserialize::default(),
                    ownership_refresh: Deserialize::default(),
                    permissions: Deserialize::default(),
                    status: Deserialize::default(),
                    subcategory: Deserialize::default(),
                    subscriptions: Deserialize::default(),
                    supported_payment_method_types: Deserialize::default(),
                    transaction_refresh: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder" => Deserialize::begin(&mut self.builder.account_holder),
                "account_numbers" => Deserialize::begin(&mut self.builder.account_numbers),
                "balance" => Deserialize::begin(&mut self.builder.balance),
                "balance_refresh" => Deserialize::begin(&mut self.builder.balance_refresh),
                "category" => Deserialize::begin(&mut self.builder.category),
                "created" => Deserialize::begin(&mut self.builder.created),
                "display_name" => Deserialize::begin(&mut self.builder.display_name),
                "id" => Deserialize::begin(&mut self.builder.id),
                "institution_name" => Deserialize::begin(&mut self.builder.institution_name),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "ownership" => Deserialize::begin(&mut self.builder.ownership),
                "ownership_refresh" => Deserialize::begin(&mut self.builder.ownership_refresh),
                "permissions" => Deserialize::begin(&mut self.builder.permissions),
                "status" => Deserialize::begin(&mut self.builder.status),
                "subcategory" => Deserialize::begin(&mut self.builder.subcategory),
                "subscriptions" => Deserialize::begin(&mut self.builder.subscriptions),
                "supported_payment_method_types" => {
                    Deserialize::begin(&mut self.builder.supported_payment_method_types)
                }
                "transaction_refresh" => Deserialize::begin(&mut self.builder.transaction_refresh),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_holder),
                Some(account_numbers),
                Some(balance),
                Some(balance_refresh),
                Some(category),
                Some(created),
                Some(display_name),
                Some(id),
                Some(institution_name),
                Some(last4),
                Some(livemode),
                Some(ownership),
                Some(ownership_refresh),
                Some(permissions),
                Some(status),
                Some(subcategory),
                Some(subscriptions),
                Some(supported_payment_method_types),
                Some(transaction_refresh),
            ) = (
                self.builder.account_holder.take(),
                self.builder.account_numbers.take(),
                self.builder.balance.take(),
                self.builder.balance_refresh.take(),
                self.builder.category.take(),
                self.builder.created,
                self.builder.display_name.take(),
                self.builder.id.take(),
                self.builder.institution_name.take(),
                self.builder.last4.take(),
                self.builder.livemode,
                self.builder.ownership.take(),
                self.builder.ownership_refresh.take(),
                self.builder.permissions.take(),
                self.builder.status.take(),
                self.builder.subcategory.take(),
                self.builder.subscriptions.take(),
                self.builder.supported_payment_method_types.take(),
                self.builder.transaction_refresh.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(FinancialConnectionsAccount {
                account_holder,
                account_numbers,
                balance,
                balance_refresh,
                category,
                created,
                display_name,
                id,
                institution_name,
                last4,
                livemode,
                ownership,
                ownership_refresh,
                permissions,
                status,
                subcategory,
                subscriptions,
                supported_payment_method_types,
                transaction_refresh,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccount {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("FinancialConnectionsAccount", 20)?;
        s.serialize_field("account_holder", &self.account_holder)?;
        s.serialize_field("account_numbers", &self.account_numbers)?;
        s.serialize_field("balance", &self.balance)?;
        s.serialize_field("balance_refresh", &self.balance_refresh)?;
        s.serialize_field("category", &self.category)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("display_name", &self.display_name)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("institution_name", &self.institution_name)?;
        s.serialize_field("last4", &self.last4)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("ownership", &self.ownership)?;
        s.serialize_field("ownership_refresh", &self.ownership_refresh)?;
        s.serialize_field("permissions", &self.permissions)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("subcategory", &self.subcategory)?;
        s.serialize_field("subscriptions", &self.subscriptions)?;
        s.serialize_field("supported_payment_method_types", &self.supported_payment_method_types)?;
        s.serialize_field("transaction_refresh", &self.transaction_refresh)?;

        s.serialize_field("object", "financial_connections.account")?;
        s.end()
    }
}
/// The type of the account. Account category is further divided in `subcategory`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsAccountCategory {
    Cash,
    Credit,
    Investment,
    Other,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsAccountCategory {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsAccountCategory::*;
        match self {
            Cash => "cash",
            Credit => "credit",
            Investment => "investment",
            Other => "other",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountCategory {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountCategory::*;
        match s {
            "cash" => Ok(Cash),
            "credit" => Ok(Credit),
            "investment" => Ok(Investment),
            "other" => Ok(Other),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsAccountCategory"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsAccountCategory)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for FinancialConnectionsAccountCategory {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountCategory> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsAccountCategory::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The list of permissions granted by this account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsAccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsAccountPermissions {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsAccountPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountPermissions {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsAccountPermissions"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsAccountPermissions)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for FinancialConnectionsAccountPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountPermissions> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsAccountPermissions::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The status of the link to the account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsAccountStatus {
    Active,
    Disconnected,
    Inactive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsAccountStatus {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsAccountStatus::*;
        match self {
            Active => "active",
            Disconnected => "disconnected",
            Inactive => "inactive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountStatus::*;
        match s {
            "active" => Ok(Active),
            "disconnected" => Ok(Disconnected),
            "inactive" => Ok(Inactive),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsAccountStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsAccountStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for FinancialConnectionsAccountStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsAccountStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsAccountSubcategory {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Other,
    Savings,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsAccountSubcategory {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsAccountSubcategory::*;
        match self {
            Checking => "checking",
            CreditCard => "credit_card",
            LineOfCredit => "line_of_credit",
            Mortgage => "mortgage",
            Other => "other",
            Savings => "savings",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountSubcategory {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountSubcategory::*;
        match s {
            "checking" => Ok(Checking),
            "credit_card" => Ok(CreditCard),
            "line_of_credit" => Ok(LineOfCredit),
            "mortgage" => Ok(Mortgage),
            "other" => Ok(Other),
            "savings" => Ok(Savings),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsAccountSubcategory"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsAccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsAccountSubcategory)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountSubcategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for FinancialConnectionsAccountSubcategory {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountSubcategory> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsAccountSubcategory::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountSubcategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The list of data refresh subscriptions requested on this account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsAccountSubscriptions {
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsAccountSubscriptions {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsAccountSubscriptions::*;
        match self {
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountSubscriptions {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountSubscriptions::*;
        match s {
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsAccountSubscriptions"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountSubscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsAccountSubscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAccountSubscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsAccountSubscriptions)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountSubscriptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for FinancialConnectionsAccountSubscriptions {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountSubscriptions> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsAccountSubscriptions::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountSubscriptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The [PaymentMethod type](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsAccountSupportedPaymentMethodTypes {
    Link,
    UsBankAccount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsAccountSupportedPaymentMethodTypes {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsAccountSupportedPaymentMethodTypes::*;
        match self {
            Link => "link",
            UsBankAccount => "us_bank_account",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountSupportedPaymentMethodTypes::*;
        match s {
            "link" => Ok(Link),
            "us_bank_account" => Ok(UsBankAccount),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsAccountSupportedPaymentMethodTypes"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsAccountSupportedPaymentMethodTypes))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<FinancialConnectionsAccountSupportedPaymentMethodTypes>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FinancialConnectionsAccountSupportedPaymentMethodTypes::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for FinancialConnectionsAccount {
    type Id = stripe_misc::FinancialConnectionsAccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(FinancialConnectionsAccountId);
