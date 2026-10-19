/// A resource for the feedback options model (for custom cancellation reasons)
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingFeedbackOption {
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::BillingFeedbackOptionId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The feedback option's status.
    pub status: stripe_shared::BillingMeterStatus,
    pub status_transitions: stripe_shared::FeedbackOptionsStatusTransitions,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingFeedbackOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingFeedbackOption").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingFeedbackOptionBuilder {
    description: Option<String>,
    id: Option<stripe_shared::BillingFeedbackOptionId>,
    livemode: Option<bool>,
    status: Option<stripe_shared::BillingMeterStatus>,
    status_transitions: Option<stripe_shared::FeedbackOptionsStatusTransitions>,
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

    impl Deserialize for BillingFeedbackOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingFeedbackOption>,
        builder: BillingFeedbackOptionBuilder,
    }

    impl Visitor for Place<BillingFeedbackOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingFeedbackOptionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingFeedbackOptionBuilder {
        type Out = BillingFeedbackOption;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "description" => Deserialize::begin(&mut self.description),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "status" => Deserialize::begin(&mut self.status),
                "status_transitions" => Deserialize::begin(&mut self.status_transitions),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                description: None,
                id: None,
                livemode: None,
                status: None,
                status_transitions: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(description),
                Some(id),
                Some(livemode),
                Some(status),
                Some(status_transitions),
            ) = (
                self.description.take(),
                self.id.take(),
                self.livemode,
                self.status.take(),
                self.status_transitions,
            )
            else {
                return None;
            };
            Some(Self::Out { description, id, livemode, status, status_transitions })
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

    impl ObjectDeser for BillingFeedbackOption {
        type Builder = BillingFeedbackOptionBuilder;
    }

    impl FromValueOpt for BillingFeedbackOption {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingFeedbackOptionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "description" => b.description = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_transitions" => b.status_transitions = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingFeedbackOption {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingFeedbackOption", 6)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;

        s.serialize_field("object", "billing.feedback_option")?;
        s.end()
    }
}
impl stripe_types::Object for BillingFeedbackOption {
    type Id = stripe_shared::BillingFeedbackOptionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BillingFeedbackOptionId);
/// The feedback option's status.
pub type BillingFeedbackOptionStatus = stripe_shared::BillingMeterStatus;
