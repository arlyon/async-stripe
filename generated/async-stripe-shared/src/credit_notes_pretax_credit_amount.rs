#[derive(Clone, Debug)]
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
                builder: CreditNotesPretaxCreditAmountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CreditNotesPretaxCreditAmountBuilder {
        type Out = CreditNotesPretaxCreditAmount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "credit_balance_transaction" => {
                    Deserialize::begin(&mut self.credit_balance_transaction)
                }
                "discount" => Deserialize::begin(&mut self.discount),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                credit_balance_transaction: Deserialize::default(),
                discount: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(credit_balance_transaction), Some(discount), Some(type_)) = (
                self.amount,
                self.credit_balance_transaction.take(),
                self.discount.take(),
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { amount, credit_balance_transaction, discount, type_ })
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

    impl ObjectDeser for CreditNotesPretaxCreditAmount {
        type Builder = CreditNotesPretaxCreditAmountBuilder;
    }

    impl FromValueOpt for CreditNotesPretaxCreditAmount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CreditNotesPretaxCreditAmountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "credit_balance_transaction" => {
                        b.credit_balance_transaction = FromValueOpt::from_value(v)
                    }
                    "discount" => b.discount = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug for CreditNotesPretaxCreditAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for CreditNotesPretaxCreditAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreditNotesPretaxCreditAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNotesPretaxCreditAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreditNotesPretaxCreditAmountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNotesPretaxCreditAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
