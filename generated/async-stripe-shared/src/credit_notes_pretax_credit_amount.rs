#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditNotesPretaxCreditAmount {
    /// The amount, in cents (or local equivalent), of the pretax credit amount.
    pub amount: i64,
    /// The credit balance transaction that was applied to get this pretax credit amount.
    pub credit_balance_transaction:
        Option<stripe_types::Expandable<stripe_shared::BillingCreditBalanceTransaction>>,
    /// The discount that was applied to get this pretax credit amount.
    pub discount: Option<stripe_types::Expandable<stripe_shared::Discount>>,
    /// Type of the pretax credit amount referenced.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: CreditNotesPretaxCreditAmountType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNotesPretaxCreditAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreditNotesPretaxCreditAmount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CreditNotesPretaxCreditAmountBuilder {
    amount: Option<i64>,
    credit_balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BillingCreditBalanceTransaction>>>,
    discount: Option<Option<stripe_types::Expandable<stripe_shared::Discount>>>,
    type_: Option<CreditNotesPretaxCreditAmountType>,
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

    impl Deserialize for CreditNotesPretaxCreditAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CreditNotesPretaxCreditAmount>,
        builder: CreditNotesPretaxCreditAmountBuilder,
    }

    impl Visitor for Place<CreditNotesPretaxCreditAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CreditNotesPretaxCreditAmountBuilder {
                    amount: Deserialize::default(),
                    credit_balance_transaction: Deserialize::default(),
                    discount: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "credit_balance_transaction" => {
                    Deserialize::begin(&mut self.builder.credit_balance_transaction)
                }
                "discount" => Deserialize::begin(&mut self.builder.discount),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(credit_balance_transaction), Some(discount), Some(type_)) = (
                self.builder.amount,
                self.builder.credit_balance_transaction.take(),
                self.builder.discount.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(CreditNotesPretaxCreditAmount {
                amount,
                credit_balance_transaction,
                discount,
                type_,
            });
            Ok(())
        }
    }
};
/// Type of the pretax credit amount referenced.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreditNotesPretaxCreditAmountType {
    CreditBalanceTransaction,
    Discount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreditNotesPretaxCreditAmountType {
    pub fn as_str(&self) -> &str {
        use CreditNotesPretaxCreditAmountType::*;
        match self {
            CreditBalanceTransaction => "credit_balance_transaction",
            Discount => "discount",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreditNotesPretaxCreditAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNotesPretaxCreditAmountType::*;
        match s {
            "credit_balance_transaction" => Ok(CreditBalanceTransaction),
            "discount" => Ok(Discount),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreditNotesPretaxCreditAmountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreditNotesPretaxCreditAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreditNotesPretaxCreditAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNotesPretaxCreditAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreditNotesPretaxCreditAmountType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNotesPretaxCreditAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CreditNotesPretaxCreditAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CreditNotesPretaxCreditAmountType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNotesPretaxCreditAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNotesPretaxCreditAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
