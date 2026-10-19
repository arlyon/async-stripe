#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionCancel {
    pub cancellation_reason: stripe_billing::PortalSubscriptionCancellationReason,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    pub mode: stripe_shared::PortalSubscriptionCancelMode,
    /// Whether to create prorations when canceling subscriptions.
    /// Possible values are `none` and `create_prorations`.
    pub proration_behavior: stripe_shared::ProrationBehavior,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalSubscriptionCancel").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalSubscriptionCancelBuilder {
    cancellation_reason: Option<stripe_billing::PortalSubscriptionCancellationReason>,
    enabled: Option<bool>,
    mode: Option<stripe_shared::PortalSubscriptionCancelMode>,
    proration_behavior: Option<stripe_shared::ProrationBehavior>,
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

    impl Deserialize for PortalSubscriptionCancel {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionCancel>,
        builder: PortalSubscriptionCancelBuilder,
    }

    impl Visitor for Place<PortalSubscriptionCancel> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalSubscriptionCancelBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalSubscriptionCancelBuilder {
        type Out = PortalSubscriptionCancel;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cancellation_reason" => Deserialize::begin(&mut self.cancellation_reason),
                "enabled" => Deserialize::begin(&mut self.enabled),
                "mode" => Deserialize::begin(&mut self.mode),
                "proration_behavior" => Deserialize::begin(&mut self.proration_behavior),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { cancellation_reason: None, enabled: None, mode: None, proration_behavior: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(cancellation_reason), Some(enabled), Some(mode), Some(proration_behavior)) = (
                self.cancellation_reason.take(),
                self.enabled,
                self.mode.take(),
                self.proration_behavior.take(),
            ) else {
                return None;
            };
            Some(Self::Out { cancellation_reason, enabled, mode, proration_behavior })
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

    impl ObjectDeser for PortalSubscriptionCancel {
        type Builder = PortalSubscriptionCancelBuilder;
    }

    impl FromValueOpt for PortalSubscriptionCancel {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalSubscriptionCancelBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "cancellation_reason" => b.cancellation_reason = FromValueOpt::from_value(v),
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "mode" => b.mode = FromValueOpt::from_value(v),
                    "proration_behavior" => b.proration_behavior = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether to cancel subscriptions immediately or at the end of the billing period.
pub type PortalSubscriptionCancelMode = stripe_shared::PortalSubscriptionCancelMode;
/// Whether to create prorations when canceling subscriptions.
/// Possible values are `none` and `create_prorations`.
pub type PortalSubscriptionCancelProrationBehavior = stripe_shared::ProrationBehavior;
