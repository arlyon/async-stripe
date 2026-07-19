/// Outcome details for this payment evaluation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationOutcome {
    pub merchant_blocked: Option<stripe_fraud::InsightsResourcesPaymentEvaluationMerchantBlocked>,
    /// The PaymentIntent ID associated with the payment evaluation.
    pub payment_intent_id: Option<String>,
    pub rejected: Option<stripe_fraud::InsightsResourcesPaymentEvaluationRejected>,
    pub succeeded: Option<stripe_fraud::InsightsResourcesPaymentEvaluationSucceeded>,
    /// Indicates the outcome of the payment evaluation.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: InsightsResourcesPaymentEvaluationOutcomeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationOutcome").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationOutcomeBuilder {
    merchant_blocked:
        Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationMerchantBlocked>>,
    payment_intent_id: Option<Option<String>>,
    rejected: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationRejected>>,
    succeeded: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationSucceeded>>,
    type_: Option<InsightsResourcesPaymentEvaluationOutcomeType>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationOutcome {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationOutcome>,
        builder: InsightsResourcesPaymentEvaluationOutcomeBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationOutcome> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationOutcomeBuilder {
                    merchant_blocked: Deserialize::default(),
                    payment_intent_id: Deserialize::default(),
                    rejected: Deserialize::default(),
                    succeeded: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "merchant_blocked" => Deserialize::begin(&mut self.builder.merchant_blocked),
                "payment_intent_id" => Deserialize::begin(&mut self.builder.payment_intent_id),
                "rejected" => Deserialize::begin(&mut self.builder.rejected),
                "succeeded" => Deserialize::begin(&mut self.builder.succeeded),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(merchant_blocked),
                Some(payment_intent_id),
                Some(rejected),
                Some(succeeded),
                Some(type_),
            ) = (
                self.builder.merchant_blocked.take(),
                self.builder.payment_intent_id.take(),
                self.builder.rejected.take(),
                self.builder.succeeded.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(InsightsResourcesPaymentEvaluationOutcome {
                merchant_blocked,
                payment_intent_id,
                rejected,
                succeeded,
                type_,
            });
            Ok(())
        }
    }
};
/// Indicates the outcome of the payment evaluation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationOutcomeType {
    Failed,
    MerchantBlocked,
    Rejected,
    Succeeded,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationOutcomeType {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationOutcomeType::*;
        match self {
            Failed => "failed",
            MerchantBlocked => "merchant_blocked",
            Rejected => "rejected",
            Succeeded => "succeeded",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationOutcomeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationOutcomeType::*;
        match s {
            "failed" => Ok(Failed),
            "merchant_blocked" => Ok(MerchantBlocked),
            "rejected" => Ok(Rejected),
            "succeeded" => Ok(Succeeded),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationOutcomeType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationOutcomeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationOutcomeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationOutcomeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InsightsResourcesPaymentEvaluationOutcomeType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationOutcomeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InsightsResourcesPaymentEvaluationOutcomeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<InsightsResourcesPaymentEvaluationOutcomeType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(InsightsResourcesPaymentEvaluationOutcomeType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationOutcomeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
