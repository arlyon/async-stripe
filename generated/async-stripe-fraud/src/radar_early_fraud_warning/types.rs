/// An early fraud warning indicates that the card issuer has notified us that a
/// charge may be fraudulent.
///
/// Related guide: [Early fraud warnings](https://docs.stripe.com/disputes/measuring#early-fraud-warnings).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RadarEarlyFraudWarning {
    /// An EFW is actionable if it has not received a dispute and has not been fully refunded.
    /// You may wish to proactively refund a charge that receives an EFW, in order to avoid receiving a dispute later.
    pub actionable: bool,
    /// ID of the charge this early fraud warning is for, optionally expanded.
    pub charge: stripe_types::Expandable<stripe_shared::Charge>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The type of fraud labelled by the issuer.
    /// One of `card_never_received`, `fraudulent_card_application`, `made_with_counterfeit_card`, `made_with_lost_card`, `made_with_stolen_card`, `misc`, `unauthorized_use_of_card`.
    pub fraud_type: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarEarlyFraudWarningId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// ID of the Payment Intent this early fraud warning is for, optionally expanded.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RadarEarlyFraudWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RadarEarlyFraudWarning").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RadarEarlyFraudWarningBuilder {
    actionable: Option<bool>,
    charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    created: Option<stripe_types::Timestamp>,
    fraud_type: Option<String>,
    id: Option<stripe_fraud::RadarEarlyFraudWarningId>,
    livemode: Option<bool>,
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
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

    impl Deserialize for RadarEarlyFraudWarning {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarEarlyFraudWarning>,
        builder: RadarEarlyFraudWarningBuilder,
    }

    impl Visitor for Place<RadarEarlyFraudWarning> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RadarEarlyFraudWarningBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RadarEarlyFraudWarningBuilder {
        type Out = RadarEarlyFraudWarning;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "actionable" => Deserialize::begin(&mut self.actionable),
                "charge" => Deserialize::begin(&mut self.charge),
                "created" => Deserialize::begin(&mut self.created),
                "fraud_type" => Deserialize::begin(&mut self.fraud_type),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "payment_intent" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.payment_intent)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                actionable: None,
                charge: None,
                created: None,
                fraud_type: None,
                id: None,
                livemode: None,
                payment_intent: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    actionable: Some(actionable),
                    charge: Some(charge),
                    created: Some(created),
                    fraud_type: Some(fraud_type),
                    id: Some(id),
                    livemode: Some(livemode),
                    payment_intent,
                } => Some(Self::Out {
                    actionable,
                    charge,
                    created,
                    fraud_type,
                    id,
                    livemode,
                    payment_intent,
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

    impl ObjectDeser for RadarEarlyFraudWarning {
        type Builder = RadarEarlyFraudWarningBuilder;
    }

    impl FromValueOpt for RadarEarlyFraudWarning {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for RadarEarlyFraudWarning {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("RadarEarlyFraudWarning", 8)?;
        s.serialize_field("actionable", &self.actionable)?;
        s.serialize_field("charge", &self.charge)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("fraud_type", &self.fraud_type)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("payment_intent", &self.payment_intent)?;

        s.serialize_field("object", "radar.early_fraud_warning")?;
        s.end()
    }
}
impl stripe_types::Object for RadarEarlyFraudWarning {
    type Id = stripe_fraud::RadarEarlyFraudWarningId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(RadarEarlyFraudWarningId);
