/// A credit balance transaction is a resource representing a transaction (either a credit or a debit) against an existing credit grant.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// ID of the test clock this credit balance transaction belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
    /// The type of credit balance transaction (credit or debit).
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: Option<BillingCreditBalanceTransactionType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditBalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingCreditBalanceTransaction").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BillingCreditBalanceTransactionBuilder {
                    created: Deserialize::default(),
                    credit: Deserialize::default(),
                    credit_grant: Deserialize::default(),
                    debit: Deserialize::default(),
                    effective_at: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    test_clock: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "credit" => Deserialize::begin(&mut self.builder.credit),
                "credit_grant" => Deserialize::begin(&mut self.builder.credit_grant),
                "debit" => Deserialize::begin(&mut self.builder.debit),
                "effective_at" => Deserialize::begin(&mut self.builder.effective_at),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "test_clock" => Deserialize::begin(&mut self.builder.test_clock),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.created,
                self.builder.credit.take(),
                self.builder.credit_grant.take(),
                self.builder.debit.take(),
                self.builder.effective_at,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.test_clock.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(BillingCreditBalanceTransaction {
                created,
                credit,
                credit_grant,
                debit,
                effective_at,
                id,
                livemode,
                test_clock,
                type_,
            });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingCreditBalanceTransactionType {
    Credit,
    Debit,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingCreditBalanceTransactionType {
    pub fn as_str(&self) -> &str {
        use BillingCreditBalanceTransactionType::*;
        match self {
            Credit => "credit",
            Debit => "debit",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingCreditBalanceTransactionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditBalanceTransactionType::*;
        match s {
            "credit" => Ok(Credit),
            "debit" => Ok(Debit),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingCreditBalanceTransactionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingCreditBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingCreditBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingCreditBalanceTransactionType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BillingCreditBalanceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingCreditBalanceTransactionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingCreditBalanceTransactionType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditBalanceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
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
