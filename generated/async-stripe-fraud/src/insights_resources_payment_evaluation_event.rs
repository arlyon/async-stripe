/// Event reported for this payment evaluation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationEvent").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationEventBuilder {
    dispute_opened: Option<stripe_fraud::InsightsResourcesPaymentEvaluationDisputeOpened>,
    early_fraud_warning_received:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived>,
    occurred_at: Option<stripe_types::Timestamp>,
    refunded: Option<stripe_fraud::InsightsResourcesPaymentEvaluationRefunded>,
    type_: Option<InsightsResourcesPaymentEvaluationEventType>,
    user_intervention_raised:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationUserInterventionRaised>,
    user_intervention_resolved:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationUserInterventionResolved>,
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
                "dispute_opened" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.dispute_opened)
                }
                "early_fraud_warning_received" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.early_fraud_warning_received,
                ),
                "occurred_at" => Deserialize::begin(&mut self.occurred_at),
                "refunded" => stripe_types::miniserde_helpers::begin_nullable(&mut self.refunded),
                "type" => Deserialize::begin(&mut self.type_),
                "user_intervention_raised" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.user_intervention_raised,
                ),
                "user_intervention_resolved" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.user_intervention_resolved,
                ),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                dispute_opened: None,
                early_fraud_warning_received: None,
                occurred_at: None,
                refunded: None,
                type_: None,
                user_intervention_raised: None,
                user_intervention_resolved: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    dispute_opened,
                    early_fraud_warning_received,
                    occurred_at: Some(occurred_at),
                    refunded,
                    type_: Some(type_),
                    user_intervention_raised,
                    user_intervention_resolved,
                } => Some(Self::Out {
                    dispute_opened,
                    early_fraud_warning_received,
                    occurred_at,
                    refunded,
                    type_,
                    user_intervention_raised,
                    user_intervention_resolved,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for InsightsResourcesPaymentEvaluationEvent {
        type Builder = InsightsResourcesPaymentEvaluationEventBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationEvent {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InsightsResourcesPaymentEvaluationEventType))
            .finish_non_exhaustive()
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
