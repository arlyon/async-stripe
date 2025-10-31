/// A Financial Connections Account represents an account that exists outside of Stripe, to which you have been granted some degree of access.
///
/// For more details see <<https://stripe.com/docs/api/financial_connections/accounts/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
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
#[doc(hidden)]
pub struct FinancialConnectionsAccountBuilder {
    account_holder: Option<Option<stripe_misc::BankConnectionsResourceAccountholder>>,
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: FinancialConnectionsAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FinancialConnectionsAccountBuilder {
        type Out = FinancialConnectionsAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder" => Deserialize::begin(&mut self.account_holder),
                "balance" => Deserialize::begin(&mut self.balance),
                "balance_refresh" => Deserialize::begin(&mut self.balance_refresh),
                "category" => Deserialize::begin(&mut self.category),
                "created" => Deserialize::begin(&mut self.created),
                "display_name" => Deserialize::begin(&mut self.display_name),
                "id" => Deserialize::begin(&mut self.id),
                "institution_name" => Deserialize::begin(&mut self.institution_name),
                "last4" => Deserialize::begin(&mut self.last4),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "ownership" => Deserialize::begin(&mut self.ownership),
                "ownership_refresh" => Deserialize::begin(&mut self.ownership_refresh),
                "permissions" => Deserialize::begin(&mut self.permissions),
                "status" => Deserialize::begin(&mut self.status),
                "subcategory" => Deserialize::begin(&mut self.subcategory),
                "subscriptions" => Deserialize::begin(&mut self.subscriptions),
                "supported_payment_method_types" => {
                    Deserialize::begin(&mut self.supported_payment_method_types)
                }
                "transaction_refresh" => Deserialize::begin(&mut self.transaction_refresh),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder: Deserialize::default(),
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder),
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
                self.account_holder.take(),
                self.balance.take(),
                self.balance_refresh,
                self.category,
                self.created,
                self.display_name.take(),
                self.id.take(),
                self.institution_name.take(),
                self.last4.take(),
                self.livemode,
                self.ownership.take(),
                self.ownership_refresh,
                self.permissions.take(),
                self.status,
                self.subcategory,
                self.subscriptions.take(),
                self.supported_payment_method_types.take(),
                self.transaction_refresh.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder,
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
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for FinancialConnectionsAccount {
        type Builder = FinancialConnectionsAccountBuilder;
    }

    impl FromValueOpt for FinancialConnectionsAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FinancialConnectionsAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder" => b.account_holder = FromValueOpt::from_value(v),
                    "balance" => b.balance = FromValueOpt::from_value(v),
                    "balance_refresh" => b.balance_refresh = FromValueOpt::from_value(v),
                    "category" => b.category = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "institution_name" => b.institution_name = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "ownership" => b.ownership = FromValueOpt::from_value(v),
                    "ownership_refresh" => b.ownership_refresh = FromValueOpt::from_value(v),
                    "permissions" => b.permissions = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "subcategory" => b.subcategory = FromValueOpt::from_value(v),
                    "subscriptions" => b.subscriptions = FromValueOpt::from_value(v),
                    "supported_payment_method_types" => {
                        b.supported_payment_method_types = FromValueOpt::from_value(v)
                    }
                    "transaction_refresh" => b.transaction_refresh = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccount {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("FinancialConnectionsAccount", 19)?;
        s.serialize_field("account_holder", &self.account_holder)?;
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountCategory::*;
        match s {
            "cash" => Ok(Cash),
            "credit" => Ok(Credit),
            "investment" => Ok(Investment),
            "other" => Ok(Other),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FinancialConnectionsAccountCategory {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountCategory> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(FinancialConnectionsAccountCategory::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsAccountCategory);
#[cfg(feature = "deserialize")]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FinancialConnectionsAccountPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountPermissions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FinancialConnectionsAccountPermissions::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsAccountPermissions);
#[cfg(feature = "deserialize")]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountStatus::*;
        match s {
            "active" => Ok(Active),
            "disconnected" => Ok(Disconnected),
            "inactive" => Ok(Inactive),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FinancialConnectionsAccountStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(FinancialConnectionsAccountStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsAccountStatus);
#[cfg(feature = "deserialize")]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountSubcategory::*;
        match s {
            "checking" => Ok(Checking),
            "credit_card" => Ok(CreditCard),
            "line_of_credit" => Ok(LineOfCredit),
            "mortgage" => Ok(Mortgage),
            "other" => Ok(Other),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountSubcategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FinancialConnectionsAccountSubcategory {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountSubcategory> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FinancialConnectionsAccountSubcategory::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsAccountSubcategory);
#[cfg(feature = "deserialize")]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountSubscriptions::*;
        match s {
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountSubscriptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FinancialConnectionsAccountSubscriptions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FinancialConnectionsAccountSubscriptions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FinancialConnectionsAccountSubscriptions::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsAccountSubscriptions);
#[cfg(feature = "deserialize")]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAccountSupportedPaymentMethodTypes::*;
        match s {
            "link" => Ok(Link),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<FinancialConnectionsAccountSupportedPaymentMethodTypes>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FinancialConnectionsAccountSupportedPaymentMethodTypes::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsAccountSupportedPaymentMethodTypes);
#[cfg(feature = "deserialize")]
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

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(FinancialConnectionsAccountId);
