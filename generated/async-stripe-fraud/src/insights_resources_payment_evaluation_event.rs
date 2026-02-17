/// Event reported for this payment evaluation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationEvent {
    pub dispute_opened: Option<stripe_fraud::InsightsResourcesPaymentEvaluationDisputeOpened>,
    pub early_fraud_warning_received:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived>,
    /// Timestamp when the event occurred.
    pub occurred_at: stripe_types::Timestamp,
    pub refunded: Option<stripe_fraud::InsightsResourcesPaymentEvaluationRefunded>,
    /// Indicates the type of event attached to the payment evaluation.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: InsightsResourcesPaymentEvaluationEventType,
    pub user_intervention_raised:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationUserInterventionRaised>,
    pub user_intervention_resolved:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationUserInterventionResolved>,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationEventBuilder {
    dispute_opened: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationDisputeOpened>>,
    early_fraud_warning_received:
        Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived>>,
    occurred_at: Option<stripe_types::Timestamp>,
    refunded: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationRefunded>>,
    type_: Option<InsightsResourcesPaymentEvaluationEventType>,
    user_intervention_raised:
        Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationUserInterventionRaised>>,
    user_intervention_resolved:
        Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationUserInterventionResolved>>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationEvent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationEvent>,
        builder: InsightsResourcesPaymentEvaluationEventBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationEvent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationEventBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationEventBuilder {
        type Out = InsightsResourcesPaymentEvaluationEvent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dispute_opened" => Deserialize::begin(&mut self.dispute_opened),
                "early_fraud_warning_received" => {
                    Deserialize::begin(&mut self.early_fraud_warning_received)
                }
                "occurred_at" => Deserialize::begin(&mut self.occurred_at),
                "refunded" => Deserialize::begin(&mut self.refunded),
                "type" => Deserialize::begin(&mut self.type_),
                "user_intervention_raised" => {
                    Deserialize::begin(&mut self.user_intervention_raised)
                }
                "user_intervention_resolved" => {
                    Deserialize::begin(&mut self.user_intervention_resolved)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                dispute_opened: Deserialize::default(),
                early_fraud_warning_received: Deserialize::default(),
                occurred_at: Deserialize::default(),
                refunded: Deserialize::default(),
                type_: Deserialize::default(),
                user_intervention_raised: Deserialize::default(),
                user_intervention_resolved: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(dispute_opened),
                Some(early_fraud_warning_received),
                Some(occurred_at),
                Some(refunded),
                Some(type_),
                Some(user_intervention_raised),
                Some(user_intervention_resolved),
            ) = (
                self.dispute_opened.take(),
                self.early_fraud_warning_received.take(),
                self.occurred_at,
                self.refunded.take(),
                self.type_.take(),
                self.user_intervention_raised.take(),
                self.user_intervention_resolved.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                dispute_opened,
                early_fraud_warning_received,
                occurred_at,
                refunded,
                type_,
                user_intervention_raised,
                user_intervention_resolved,
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationEvent {
        type Builder = InsightsResourcesPaymentEvaluationEventBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationEvent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationEventBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dispute_opened" => b.dispute_opened = FromValueOpt::from_value(v),
                    "early_fraud_warning_received" => {
                        b.early_fraud_warning_received = FromValueOpt::from_value(v)
                    }
                    "occurred_at" => b.occurred_at = FromValueOpt::from_value(v),
                    "refunded" => b.refunded = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "user_intervention_raised" => {
                        b.user_intervention_raised = FromValueOpt::from_value(v)
                    }
                    "user_intervention_resolved" => {
                        b.user_intervention_resolved = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates the type of event attached to the payment evaluation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationEventType {
    DisputeOpened,
    EarlyFraudWarningReceived,
    Refunded,
    UserInterventionRaised,
    UserInterventionResolved,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationEventType {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationEventType::*;
        match self {
            DisputeOpened => "dispute_opened",
            EarlyFraudWarningReceived => "early_fraud_warning_received",
            Refunded => "refunded",
            UserInterventionRaised => "user_intervention_raised",
            UserInterventionResolved => "user_intervention_resolved",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationEventType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationEventType::*;
        match s {
            "dispute_opened" => Ok(DisputeOpened),
            "early_fraud_warning_received" => Ok(EarlyFraudWarningReceived),
            "refunded" => Ok(Refunded),
            "user_intervention_raised" => Ok(UserInterventionRaised),
            "user_intervention_resolved" => Ok(UserInterventionResolved),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationEventType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationEventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationEventType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InsightsResourcesPaymentEvaluationEventType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(InsightsResourcesPaymentEvaluationEventType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InsightsResourcesPaymentEvaluationEventType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
