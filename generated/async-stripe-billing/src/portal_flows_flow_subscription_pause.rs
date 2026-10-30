#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsFlowSubscriptionPause {
    /// The ID of the subscription to be paused.
    pub subscription: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsFlowSubscriptionPause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFlowsFlowSubscriptionPause").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalFlowsFlowSubscriptionPauseBuilder {
    subscription: Option<String>,
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

    impl Deserialize for PortalFlowsFlowSubscriptionPause {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlowSubscriptionPause>,
        builder: PortalFlowsFlowSubscriptionPauseBuilder,
    }

    impl Visitor for Place<PortalFlowsFlowSubscriptionPause> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsFlowSubscriptionPauseBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsFlowSubscriptionPauseBuilder {
        type Out = PortalFlowsFlowSubscriptionPause;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "subscription" => Deserialize::begin(&mut self.subscription),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { subscription: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(subscription),) = (self.subscription.take(),) else {
                return None;
            };
            Some(Self::Out { subscription })
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

    impl ObjectDeser for PortalFlowsFlowSubscriptionPause {
        type Builder = PortalFlowsFlowSubscriptionPauseBuilder;
    }

    impl FromValueOpt for PortalFlowsFlowSubscriptionPause {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsFlowSubscriptionPauseBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "subscription" => b.subscription = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
