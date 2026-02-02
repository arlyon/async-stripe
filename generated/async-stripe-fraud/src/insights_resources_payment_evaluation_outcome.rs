/// Outcome details for this payment evaluation.
#[derive(Clone, Debug)]
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
                builder: InsightsResourcesPaymentEvaluationOutcomeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationOutcomeBuilder {
        type Out = InsightsResourcesPaymentEvaluationOutcome;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "merchant_blocked" => Deserialize::begin(&mut self.merchant_blocked),
                "payment_intent_id" => Deserialize::begin(&mut self.payment_intent_id),
                "rejected" => Deserialize::begin(&mut self.rejected),
                "succeeded" => Deserialize::begin(&mut self.succeeded),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                merchant_blocked: Deserialize::default(),
                payment_intent_id: Deserialize::default(),
                rejected: Deserialize::default(),
                succeeded: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(merchant_blocked),
                Some(payment_intent_id),
                Some(rejected),
                Some(succeeded),
                Some(type_),
            ) = (
                self.merchant_blocked.take(),
                self.payment_intent_id.take(),
                self.rejected.take(),
                self.succeeded.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { merchant_blocked, payment_intent_id, rejected, succeeded, type_ })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationOutcome {
        type Builder = InsightsResourcesPaymentEvaluationOutcomeBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationOutcome {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationOutcomeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "merchant_blocked" => b.merchant_blocked = FromValueOpt::from_value(v),
                    "payment_intent_id" => b.payment_intent_id = FromValueOpt::from_value(v),
                    "rejected" => b.rejected = FromValueOpt::from_value(v),
                    "succeeded" => b.succeeded = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationOutcomeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationOutcomeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InsightsResourcesPaymentEvaluationOutcomeType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(InsightsResourcesPaymentEvaluationOutcomeType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InsightsResourcesPaymentEvaluationOutcomeType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationOutcomeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
