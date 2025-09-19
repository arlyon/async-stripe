#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceBalanceCredit {
    pub amount: stripe_shared::BillingCreditGrantsResourceAmount,
    /// Details of the invoice to which the reinstated credits were originally applied.
    /// Only present if `type` is `credits_application_invoice_voided`.
    pub credits_application_invoice_voided:
        Option<stripe_shared::BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoided>,
    /// The type of credit transaction.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BillingCreditGrantsResourceBalanceCreditType,
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceBalanceCreditBuilder {
    amount: Option<stripe_shared::BillingCreditGrantsResourceAmount>,
    credits_application_invoice_voided: Option<
        Option<stripe_shared::BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoided>,
    >,
    type_: Option<BillingCreditGrantsResourceBalanceCreditType>,
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

    impl Deserialize for BillingCreditGrantsResourceBalanceCredit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditGrantsResourceBalanceCredit>,
        builder: BillingCreditGrantsResourceBalanceCreditBuilder,
    }

    impl Visitor for Place<BillingCreditGrantsResourceBalanceCredit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditGrantsResourceBalanceCreditBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditGrantsResourceBalanceCreditBuilder {
        type Out = BillingCreditGrantsResourceBalanceCredit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "credits_application_invoice_voided" => {
                    Deserialize::begin(&mut self.credits_application_invoice_voided)
                }
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                credits_application_invoice_voided: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(credits_application_invoice_voided), Some(type_)) =
                (self.amount.take(), self.credits_application_invoice_voided.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { amount, credits_application_invoice_voided, type_ })
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

    impl ObjectDeser for BillingCreditGrantsResourceBalanceCredit {
        type Builder = BillingCreditGrantsResourceBalanceCreditBuilder;
    }

    impl FromValueOpt for BillingCreditGrantsResourceBalanceCredit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditGrantsResourceBalanceCreditBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "credits_application_invoice_voided" => {
                        b.credits_application_invoice_voided = FromValueOpt::from_value(v)
                    }
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of credit transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingCreditGrantsResourceBalanceCreditType {
    CreditsApplicationInvoiceVoided,
    CreditsGranted,
}
impl BillingCreditGrantsResourceBalanceCreditType {
    pub fn as_str(self) -> &'static str {
        use BillingCreditGrantsResourceBalanceCreditType::*;
        match self {
            CreditsApplicationInvoiceVoided => "credits_application_invoice_voided",
            CreditsGranted => "credits_granted",
        }
    }
}

impl std::str::FromStr for BillingCreditGrantsResourceBalanceCreditType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditGrantsResourceBalanceCreditType::*;
        match s {
            "credits_application_invoice_voided" => Ok(CreditsApplicationInvoiceVoided),
            "credits_granted" => Ok(CreditsGranted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingCreditGrantsResourceBalanceCreditType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingCreditGrantsResourceBalanceCreditType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingCreditGrantsResourceBalanceCreditType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingCreditGrantsResourceBalanceCreditType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingCreditGrantsResourceBalanceCreditType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingCreditGrantsResourceBalanceCreditType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingCreditGrantsResourceBalanceCreditType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditGrantsResourceBalanceCreditType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BillingCreditGrantsResourceBalanceCreditType",
            )
        })
    }
}
