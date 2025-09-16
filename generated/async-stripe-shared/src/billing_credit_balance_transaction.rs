/// A credit balance transaction is a resource representing a transaction (either a credit or a debit) against an existing credit grant.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditBalanceTransaction {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Credit details for this credit balance transaction. Only present if type is `credit`.
    pub credit: Option<stripe_shared::BillingCreditGrantsResourceBalanceCredit>,
    /// The credit grant associated with this credit balance transaction.
    pub credit_grant: stripe_types::Expandable<stripe_shared::BillingCreditGrant>,
    /// Debit details for this credit balance transaction. Only present if type is `debit`.
    pub debit: Option<stripe_shared::BillingCreditGrantsResourceBalanceDebit>,
    /// The effective time of this credit balance transaction.
    pub effective_at: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_shared::BillingCreditBalanceTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the test clock this credit balance transaction belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
    /// The type of credit balance transaction (credit or debit).
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: Option<BillingCreditBalanceTransactionType>,
}
#[doc(hidden)]
pub struct BillingCreditBalanceTransactionBuilder {
    created: Option<stripe_types::Timestamp>,
    credit: Option<Option<stripe_shared::BillingCreditGrantsResourceBalanceCredit>>,
    credit_grant: Option<stripe_types::Expandable<stripe_shared::BillingCreditGrant>>,
    debit: Option<Option<stripe_shared::BillingCreditGrantsResourceBalanceDebit>>,
    effective_at: Option<stripe_types::Timestamp>,
    id: Option<stripe_shared::BillingCreditBalanceTransactionId>,
    livemode: Option<bool>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
    type_: Option<Option<BillingCreditBalanceTransactionType>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BillingCreditBalanceTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditBalanceTransaction>,
        builder: BillingCreditBalanceTransactionBuilder,
    }

    impl Visitor for Place<BillingCreditBalanceTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditBalanceTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditBalanceTransactionBuilder {
        type Out = BillingCreditBalanceTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "credit" => Deserialize::begin(&mut self.credit),
                "credit_grant" => Deserialize::begin(&mut self.credit_grant),
                "debit" => Deserialize::begin(&mut self.debit),
                "effective_at" => Deserialize::begin(&mut self.effective_at),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "test_clock" => Deserialize::begin(&mut self.test_clock),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                credit: Deserialize::default(),
                credit_grant: Deserialize::default(),
                debit: Deserialize::default(),
                effective_at: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                test_clock: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(credit),
                Some(credit_grant),
                Some(debit),
                Some(effective_at),
                Some(id),
                Some(livemode),
                Some(test_clock),
                Some(type_),
            ) = (
                self.created,
                self.credit.take(),
                self.credit_grant.take(),
                self.debit.take(),
                self.effective_at,
                self.id.take(),
                self.livemode,
                self.test_clock.take(),
                self.type_,
            )
            else {
                return None;
            };
            Some(Self::Out {
                created,
                credit,
                credit_grant,
                debit,
                effective_at,
                id,
                livemode,
                test_clock,
                type_,
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

    impl ObjectDeser for BillingCreditBalanceTransaction {
        type Builder = BillingCreditBalanceTransactionBuilder;
    }

    impl FromValueOpt for BillingCreditBalanceTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditBalanceTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "credit" => b.credit = FromValueOpt::from_value(v),
                    "credit_grant" => b.credit_grant = FromValueOpt::from_value(v),
                    "debit" => b.debit = FromValueOpt::from_value(v),
                    "effective_at" => b.effective_at = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "test_clock" => b.test_clock = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingCreditBalanceTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingCreditBalanceTransaction", 10)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("credit", &self.credit)?;
        s.serialize_field("credit_grant", &self.credit_grant)?;
        s.serialize_field("debit", &self.debit)?;
        s.serialize_field("effective_at", &self.effective_at)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("test_clock", &self.test_clock)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "billing.credit_balance_transaction")?;
        s.end()
    }
}
/// The type of credit balance transaction (credit or debit).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingCreditBalanceTransactionType {
    Credit,
    Debit,
}
impl BillingCreditBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        use BillingCreditBalanceTransactionType::*;
        match self {
            Credit => "credit",
            Debit => "debit",
        }
    }
}

impl std::str::FromStr for BillingCreditBalanceTransactionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditBalanceTransactionType::*;
        match s {
            "credit" => Ok(Credit),
            "debit" => Ok(Debit),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingCreditBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingCreditBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingCreditBalanceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingCreditBalanceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingCreditBalanceTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BillingCreditBalanceTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingCreditBalanceTransactionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditBalanceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BillingCreditBalanceTransactionType")
        })
    }
}
impl stripe_types::Object for BillingCreditBalanceTransaction {
    type Id = stripe_shared::BillingCreditBalanceTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BillingCreditBalanceTransactionId);
