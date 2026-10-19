#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionCancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The IDs of custom feedback options configured for this cancellation reason.
    pub feedback_options:
        Option<Vec<stripe_types::Expandable<stripe_shared::BillingFeedbackOption>>>,
    /// Which cancellation reasons will be given as options to the customer.
    pub options: Vec<stripe_shared::CancellationDetailsFeedback>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalSubscriptionCancellationReason").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalSubscriptionCancellationReasonBuilder {
    enabled: Option<bool>,
    feedback_options:
        Option<Option<Vec<stripe_types::Expandable<stripe_shared::BillingFeedbackOption>>>>,
    options: Option<Vec<stripe_shared::CancellationDetailsFeedback>>,
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

    impl Deserialize for PortalSubscriptionCancellationReason {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionCancellationReason>,
        builder: PortalSubscriptionCancellationReasonBuilder,
    }

    impl Visitor for Place<PortalSubscriptionCancellationReason> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalSubscriptionCancellationReasonBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalSubscriptionCancellationReasonBuilder {
        type Out = PortalSubscriptionCancellationReason;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),
                "feedback_options" => Deserialize::begin(&mut self.feedback_options),
                "options" => Deserialize::begin(&mut self.options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enabled: None, feedback_options: Some(None), options: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(enabled), Some(feedback_options), Some(options)) =
                (self.enabled, self.feedback_options.take(), self.options.take())
            else {
                return None;
            };
            Some(Self::Out { enabled, feedback_options, options })
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

    impl ObjectDeser for PortalSubscriptionCancellationReason {
        type Builder = PortalSubscriptionCancellationReasonBuilder;
    }

    impl FromValueOpt for PortalSubscriptionCancellationReason {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalSubscriptionCancellationReasonBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "feedback_options" => b.feedback_options = FromValueOpt::from_value(v),
                    "options" => b.options = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Which cancellation reasons will be given as options to the customer.
pub type PortalSubscriptionCancellationReasonOptions = stripe_shared::CancellationDetailsFeedback;
