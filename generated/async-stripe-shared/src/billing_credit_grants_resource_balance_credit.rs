#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditGrantsResourceBalanceCredit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingCreditGrantsResourceBalanceCredit").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BillingCreditGrantsResourceBalanceCreditBuilder {
                    amount: Deserialize::default(),
                    credits_application_invoice_voided: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "credits_application_invoice_voided" => {
                    Deserialize::begin(&mut self.builder.credits_application_invoice_voided)
                }
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(credits_application_invoice_voided), Some(type_)) = (
                self.builder.amount.take(),
                self.builder.credits_application_invoice_voided.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(BillingCreditGrantsResourceBalanceCredit {
                amount,
                credits_application_invoice_voided,
                type_,
            });
            Ok(())
        }
    }
};
/// The type of credit transaction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingCreditGrantsResourceBalanceCreditType {
    CreditsApplicationInvoiceVoided,
    CreditsGranted,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingCreditGrantsResourceBalanceCreditType {
    pub fn as_str(&self) -> &str {
        use BillingCreditGrantsResourceBalanceCreditType::*;
        match self {
            CreditsApplicationInvoiceVoided => "credits_application_invoice_voided",
            CreditsGranted => "credits_granted",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingCreditGrantsResourceBalanceCreditType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditGrantsResourceBalanceCreditType::*;
        match s {
            "credits_application_invoice_voided" => Ok(CreditsApplicationInvoiceVoided),
            "credits_granted" => Ok(CreditsGranted),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingCreditGrantsResourceBalanceCreditType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingCreditGrantsResourceBalanceCreditType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingCreditGrantsResourceBalanceCreditType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditGrantsResourceBalanceCreditType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingCreditGrantsResourceBalanceCreditType))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BillingCreditGrantsResourceBalanceCreditType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingCreditGrantsResourceBalanceCreditType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BillingCreditGrantsResourceBalanceCreditType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditGrantsResourceBalanceCreditType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
