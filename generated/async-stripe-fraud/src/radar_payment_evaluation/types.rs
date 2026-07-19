/// Payment Evaluations represent the risk lifecycle of an externally processed payment.
/// It includes the Radar risk score from Stripe, payment outcome taken by the merchant or processor, and any post transaction events, such as refunds or disputes.
/// See the [Radar API guide](/radar/multiprocessor) for integration steps.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RadarPaymentEvaluation {
    pub client_device_metadata_details:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationClientDeviceMetadata>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created_at: stripe_types::Timestamp,
    pub customer_details: Option<stripe_fraud::InsightsResourcesPaymentEvaluationCustomerDetails>,
    /// Event information associated with the payment evaluation, such as refunds, dispute, early fraud warnings, or user interventions.
    pub events: Option<Vec<stripe_fraud::InsightsResourcesPaymentEvaluationEvent>>,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarPaymentEvaluationId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Indicates the final outcome for the payment evaluation.
    pub outcome: Option<stripe_fraud::InsightsResourcesPaymentEvaluationOutcome>,
    pub payment_details: Option<stripe_fraud::InsightsResourcesPaymentEvaluationPaymentDetails>,
    /// Recommended action based on the score of the fraudulent_payment signal.
    /// Possible values are `block` and `continue`.
    pub recommended_action: RadarPaymentEvaluationRecommendedAction,
    pub signals: stripe_fraud::InsightsResourcesPaymentEvaluationSignals,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RadarPaymentEvaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RadarPaymentEvaluation").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RadarPaymentEvaluationBuilder {
    client_device_metadata_details:
        Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationClientDeviceMetadata>>,
    created_at: Option<stripe_types::Timestamp>,
    customer_details:
        Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationCustomerDetails>>,
    events: Option<Option<Vec<stripe_fraud::InsightsResourcesPaymentEvaluationEvent>>>,
    id: Option<stripe_fraud::RadarPaymentEvaluationId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    outcome: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationOutcome>>,
    payment_details: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationPaymentDetails>>,
    recommended_action: Option<RadarPaymentEvaluationRecommendedAction>,
    signals: Option<stripe_fraud::InsightsResourcesPaymentEvaluationSignals>,
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

    impl Deserialize for RadarPaymentEvaluation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarPaymentEvaluation>,
        builder: RadarPaymentEvaluationBuilder,
    }

    impl Visitor for Place<RadarPaymentEvaluation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RadarPaymentEvaluationBuilder {
                    client_device_metadata_details: Deserialize::default(),
                    created_at: Deserialize::default(),
                    customer_details: Deserialize::default(),
                    events: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    outcome: Deserialize::default(),
                    payment_details: Deserialize::default(),
                    recommended_action: Deserialize::default(),
                    signals: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "client_device_metadata_details" => {
                    Deserialize::begin(&mut self.builder.client_device_metadata_details)
                }
                "created_at" => Deserialize::begin(&mut self.builder.created_at),
                "customer_details" => Deserialize::begin(&mut self.builder.customer_details),
                "events" => Deserialize::begin(&mut self.builder.events),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "outcome" => Deserialize::begin(&mut self.builder.outcome),
                "payment_details" => Deserialize::begin(&mut self.builder.payment_details),
                "recommended_action" => Deserialize::begin(&mut self.builder.recommended_action),
                "signals" => Deserialize::begin(&mut self.builder.signals),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(client_device_metadata_details),
                Some(created_at),
                Some(customer_details),
                Some(events),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(outcome),
                Some(payment_details),
                Some(recommended_action),
                Some(signals),
            ) = (
                self.builder.client_device_metadata_details.take(),
                self.builder.created_at,
                self.builder.customer_details.take(),
                self.builder.events.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.outcome.take(),
                self.builder.payment_details.take(),
                self.builder.recommended_action.take(),
                self.builder.signals.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(RadarPaymentEvaluation {
                client_device_metadata_details,
                created_at,
                customer_details,
                events,
                id,
                livemode,
                metadata,
                outcome,
                payment_details,
                recommended_action,
                signals,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for RadarPaymentEvaluation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("RadarPaymentEvaluation", 12)?;
        s.serialize_field("client_device_metadata_details", &self.client_device_metadata_details)?;
        s.serialize_field("created_at", &self.created_at)?;
        s.serialize_field("customer_details", &self.customer_details)?;
        s.serialize_field("events", &self.events)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("outcome", &self.outcome)?;
        s.serialize_field("payment_details", &self.payment_details)?;
        s.serialize_field("recommended_action", &self.recommended_action)?;
        s.serialize_field("signals", &self.signals)?;

        s.serialize_field("object", "radar.payment_evaluation")?;
        s.end()
    }
}
/// Recommended action based on the score of the fraudulent_payment signal.
/// Possible values are `block` and `continue`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RadarPaymentEvaluationRecommendedAction {
    Block,
    Continue,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RadarPaymentEvaluationRecommendedAction {
    pub fn as_str(&self) -> &str {
        use RadarPaymentEvaluationRecommendedAction::*;
        match self {
            Block => "block",
            Continue => "continue",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RadarPaymentEvaluationRecommendedAction {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RadarPaymentEvaluationRecommendedAction::*;
        match s {
            "block" => Ok(Block),
            "continue" => Ok(Continue),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "RadarPaymentEvaluationRecommendedAction"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RadarPaymentEvaluationRecommendedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for RadarPaymentEvaluationRecommendedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RadarPaymentEvaluationRecommendedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(RadarPaymentEvaluationRecommendedAction)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RadarPaymentEvaluationRecommendedAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for RadarPaymentEvaluationRecommendedAction {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<RadarPaymentEvaluationRecommendedAction> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RadarPaymentEvaluationRecommendedAction::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RadarPaymentEvaluationRecommendedAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for RadarPaymentEvaluation {
    type Id = stripe_fraud::RadarPaymentEvaluationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(RadarPaymentEvaluationId);
