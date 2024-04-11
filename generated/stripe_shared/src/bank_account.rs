/// These bank accounts are payment methods on `Customer` objects.
///
/// On the other hand [External Accounts](https://stripe.com/docs/api#external_accounts) are transfer
/// destinations on `Account` objects for [Custom accounts](https://stripe.com/docs/connect/custom-accounts).
/// They can be bank accounts or debit cards as well, and are documented in the links above.
///
/// Related guide: [Bank debits and transfers](https://stripe.com/docs/payments/bank-debits-transfers)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankAccount {
    /// The ID of the account that the bank account is associated with.
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: Option<String>,
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    pub account_holder_type: Option<String>,
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    pub account_type: Option<String>,
    /// A set of available payout methods for this bank account.
    /// Only values from this set should be passed as the `method` when creating a payout.
    pub available_payout_methods: Option<Vec<BankAccountAvailablePayoutMethods>>,
    /// Name of the bank associated with the routing number (e.g., `WELLS FARGO`).
    pub bank_name: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: String,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: stripe_types::Currency,
    /// The ID of the customer that the bank account is associated with.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Whether this bank account is the default external account for its currency.
    pub default_for_currency: Option<bool>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Information about the [upcoming new requirements for the bank account](https://stripe.com/docs/connect/custom-accounts/future-requirements), including what information needs to be collected, and by when.
    pub future_requirements: Option<stripe_shared::ExternalAccountRequirements>,
    /// Unique identifier for the object.
    pub id: stripe_shared::BankAccountId,
    /// The last four digits of the bank account number.
    pub last4: String,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Information about the requirements for the bank account, including what information needs to be collected.
    pub requirements: Option<stripe_shared::ExternalAccountRequirements>,
    /// The routing transit number for the bank account.
    pub routing_number: Option<String>,
    /// For bank accounts, possible values are `new`, `validated`, `verified`, `verification_failed`, or `errored`.
    /// A bank account that hasn't had any activity or validation performed is `new`.
    /// If Stripe can determine that the bank account exists, its status will be `validated`.
    /// Note that there often isn’t enough information to know (e.g., for smaller credit unions), and the validation is not always run.
    /// If customer bank account verification has succeeded, the bank account status will be `verified`.
    /// If the verification failed for any reason, such as microdeposit failure, the status will be `verification_failed`.
    /// If a payout sent to this bank account fails, we'll set the status to `errored` and will not continue to send [scheduled payouts](https://stripe.com/docs/payouts#payout-schedule) until the bank details are updated.
    ///
    /// For external accounts, possible values are `new`, `errored` and `verification_failed`.
    /// If a payouts fails, the status is set to `errored` and scheduled payouts are stopped until account details are updated.
    /// In India, if we can't [verify the owner of the bank account](https://support.stripe.com/questions/bank-account-ownership-verification), we'll set the status to `verification_failed`.
    /// Other validations aren't run against external accounts because they're only used for payouts.
    /// This means the other statuses don't apply.
    pub status: String,
}
#[doc(hidden)]
pub struct BankAccountBuilder {
    account: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    account_holder_name: Option<Option<String>>,
    account_holder_type: Option<Option<String>>,
    account_type: Option<Option<String>>,
    available_payout_methods: Option<Option<Vec<BankAccountAvailablePayoutMethods>>>,
    bank_name: Option<Option<String>>,
    country: Option<String>,
    currency: Option<stripe_types::Currency>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    default_for_currency: Option<Option<bool>>,
    fingerprint: Option<Option<String>>,
    future_requirements: Option<Option<stripe_shared::ExternalAccountRequirements>>,
    id: Option<stripe_shared::BankAccountId>,
    last4: Option<String>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    requirements: Option<Option<stripe_shared::ExternalAccountRequirements>>,
    routing_number: Option<Option<String>>,
    status: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankAccount>,
        builder: BankAccountBuilder,
    }

    impl Visitor for Place<BankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankAccountBuilder {
        type Out = BankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "account_holder_name" => Deserialize::begin(&mut self.account_holder_name),
                "account_holder_type" => Deserialize::begin(&mut self.account_holder_type),
                "account_type" => Deserialize::begin(&mut self.account_type),
                "available_payout_methods" => {
                    Deserialize::begin(&mut self.available_payout_methods)
                }
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "country" => Deserialize::begin(&mut self.country),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "default_for_currency" => Deserialize::begin(&mut self.default_for_currency),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "future_requirements" => Deserialize::begin(&mut self.future_requirements),
                "id" => Deserialize::begin(&mut self.id),
                "last4" => Deserialize::begin(&mut self.last4),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "requirements" => Deserialize::begin(&mut self.requirements),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                account_holder_name: Deserialize::default(),
                account_holder_type: Deserialize::default(),
                account_type: Deserialize::default(),
                available_payout_methods: Deserialize::default(),
                bank_name: Deserialize::default(),
                country: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                default_for_currency: Deserialize::default(),
                fingerprint: Deserialize::default(),
                future_requirements: Deserialize::default(),
                id: Deserialize::default(),
                last4: Deserialize::default(),
                metadata: Deserialize::default(),
                requirements: Deserialize::default(),
                routing_number: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account: self.account.take()?,
                account_holder_name: self.account_holder_name.take()?,
                account_holder_type: self.account_holder_type.take()?,
                account_type: self.account_type.take()?,
                available_payout_methods: self.available_payout_methods.take()?,
                bank_name: self.bank_name.take()?,
                country: self.country.take()?,
                currency: self.currency?,
                customer: self.customer.take()?,
                default_for_currency: self.default_for_currency?,
                fingerprint: self.fingerprint.take()?,
                future_requirements: self.future_requirements.take()?,
                id: self.id.take()?,
                last4: self.last4.take()?,
                metadata: self.metadata.take()?,
                requirements: self.requirements.take()?,
                routing_number: self.routing_number.take()?,
                status: self.status.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BankAccount {
        type Builder = BankAccountBuilder;
    }

    impl FromValueOpt for BankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = Some(FromValueOpt::from_value(v)?),
                    "account_holder_name" => {
                        b.account_holder_name = Some(FromValueOpt::from_value(v)?)
                    }
                    "account_holder_type" => {
                        b.account_holder_type = Some(FromValueOpt::from_value(v)?)
                    }
                    "account_type" => b.account_type = Some(FromValueOpt::from_value(v)?),
                    "available_payout_methods" => {
                        b.available_payout_methods = Some(FromValueOpt::from_value(v)?)
                    }
                    "bank_name" => b.bank_name = Some(FromValueOpt::from_value(v)?),
                    "country" => b.country = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "default_for_currency" => {
                        b.default_for_currency = Some(FromValueOpt::from_value(v)?)
                    }
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "future_requirements" => {
                        b.future_requirements = Some(FromValueOpt::from_value(v)?)
                    }
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "requirements" => b.requirements = Some(FromValueOpt::from_value(v)?),
                    "routing_number" => b.routing_number = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BankAccount {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BankAccount", 19)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("account_holder_name", &self.account_holder_name)?;
        s.serialize_field("account_holder_type", &self.account_holder_type)?;
        s.serialize_field("account_type", &self.account_type)?;
        s.serialize_field("available_payout_methods", &self.available_payout_methods)?;
        s.serialize_field("bank_name", &self.bank_name)?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("default_for_currency", &self.default_for_currency)?;
        s.serialize_field("fingerprint", &self.fingerprint)?;
        s.serialize_field("future_requirements", &self.future_requirements)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("last4", &self.last4)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("requirements", &self.requirements)?;
        s.serialize_field("routing_number", &self.routing_number)?;
        s.serialize_field("status", &self.status)?;

        s.serialize_field("object", "bank_account")?;
        s.end()
    }
}
/// A set of available payout methods for this bank account.
/// Only values from this set should be passed as the `method` when creating a payout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankAccountAvailablePayoutMethods {
    Instant,
    Standard,
}
impl BankAccountAvailablePayoutMethods {
    pub fn as_str(self) -> &'static str {
        use BankAccountAvailablePayoutMethods::*;
        match self {
            Instant => "instant",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for BankAccountAvailablePayoutMethods {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankAccountAvailablePayoutMethods::*;
        match s {
            "instant" => Ok(Instant),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BankAccountAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankAccountAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankAccountAvailablePayoutMethods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BankAccountAvailablePayoutMethods {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BankAccountAvailablePayoutMethods> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BankAccountAvailablePayoutMethods::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BankAccountAvailablePayoutMethods);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankAccountAvailablePayoutMethods {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BankAccountAvailablePayoutMethods")
        })
    }
}
impl stripe_types::Object for BankAccount {
    type Id = stripe_shared::BankAccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(BankAccountId);
