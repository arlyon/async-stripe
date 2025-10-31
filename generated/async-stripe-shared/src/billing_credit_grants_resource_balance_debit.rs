#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceBalanceDebit {
    pub amount: stripe_shared::BillingCreditGrantsResourceAmount,
    /// Details of how the billing credits were applied to an invoice.
    /// Only present if `type` is `credits_applied`.
    pub credits_applied: Option<stripe_shared::BillingCreditGrantsResourceBalanceCreditsApplied>,
    /// The type of debit transaction.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BillingCreditGrantsResourceBalanceDebitType,
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceBalanceDebitBuilder {
    amount: Option<stripe_shared::BillingCreditGrantsResourceAmount>,
    credits_applied:
        Option<Option<stripe_shared::BillingCreditGrantsResourceBalanceCreditsApplied>>,
    type_: Option<BillingCreditGrantsResourceBalanceDebitType>,
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

    impl Deserialize for BillingCreditGrantsResourceBalanceDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditGrantsResourceBalanceDebit>,
        builder: BillingCreditGrantsResourceBalanceDebitBuilder,
    }

    impl Visitor for Place<BillingCreditGrantsResourceBalanceDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditGrantsResourceBalanceDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditGrantsResourceBalanceDebitBuilder {
        type Out = BillingCreditGrantsResourceBalanceDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "credits_applied" => Deserialize::begin(&mut self.credits_applied),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                credits_applied: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(credits_applied), Some(type_)) =
                (self.amount.take(), self.credits_applied.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { amount, credits_applied, type_ })
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

    impl ObjectDeser for BillingCreditGrantsResourceBalanceDebit {
        type Builder = BillingCreditGrantsResourceBalanceDebitBuilder;
    }

    impl FromValueOpt for BillingCreditGrantsResourceBalanceDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditGrantsResourceBalanceDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "credits_applied" => b.credits_applied = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of debit transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingCreditGrantsResourceBalanceDebitType {
    CreditsApplied,
    CreditsExpired,
    CreditsVoided,
}
impl BillingCreditGrantsResourceBalanceDebitType {
    pub fn as_str(self) -> &'static str {
        use BillingCreditGrantsResourceBalanceDebitType::*;
        match self {
            CreditsApplied => "credits_applied",
            CreditsExpired => "credits_expired",
            CreditsVoided => "credits_voided",
        }
    }
}

impl std::str::FromStr for BillingCreditGrantsResourceBalanceDebitType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditGrantsResourceBalanceDebitType::*;
        match s {
            "credits_applied" => Ok(CreditsApplied),
            "credits_expired" => Ok(CreditsExpired),
            "credits_voided" => Ok(CreditsVoided),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingCreditGrantsResourceBalanceDebitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingCreditGrantsResourceBalanceDebitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingCreditGrantsResourceBalanceDebitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingCreditGrantsResourceBalanceDebitType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingCreditGrantsResourceBalanceDebitType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingCreditGrantsResourceBalanceDebitType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingCreditGrantsResourceBalanceDebitType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditGrantsResourceBalanceDebitType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BillingCreditGrantsResourceBalanceDebitType",
            )
        })
    }
}
