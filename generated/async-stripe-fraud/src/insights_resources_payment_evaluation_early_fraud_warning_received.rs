/// Early Fraud Warning Received event details attached to this payment evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived {
    /// The type of fraud labeled by the issuer.
    pub fraud_type: InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedBuilder {
    fraud_type: Option<InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived>,
        builder: InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedBuilder {
        type Out = InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fraud_type" => Deserialize::begin(&mut self.fraud_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { fraud_type: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(fraud_type),) = (self.fraud_type.take(),) else {
                return None;
            };
            Some(Self::Out { fraud_type })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived {
        type Builder = InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fraud_type" => b.fraud_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of fraud labeled by the issuer.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    MadeWithLostCard,
    MadeWithStolenCard,
    Other,
    UnauthorizedUseOfCard,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::*;
        match self {
            MadeWithLostCard => "made_with_lost_card",
            MadeWithStolenCard => "made_with_stolen_card",
            Other => "other",
            UnauthorizedUseOfCard => "unauthorized_use_of_card",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::*;
        match s {
            "made_with_lost_card" => Ok(MadeWithLostCard),
            "made_with_stolen_card" => Ok(MadeWithStolenCard),
            "other" => Ok(Other),
            "unauthorized_use_of_card" => Ok(UnauthorizedUseOfCard),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
