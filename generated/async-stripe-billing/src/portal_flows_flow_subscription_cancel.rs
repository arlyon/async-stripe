#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsFlowSubscriptionCancel {
    /// Specify a retention strategy to be used in the cancellation flow.
    pub retention: Option<stripe_billing::PortalFlowsRetention>,
    /// The ID of the subscription to be canceled.
    pub subscription: String,
}
#[doc(hidden)]
pub struct PortalFlowsFlowSubscriptionCancelBuilder {
    retention: Option<Option<stripe_billing::PortalFlowsRetention>>,
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

    impl Deserialize for PortalFlowsFlowSubscriptionCancel {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlowSubscriptionCancel>,
        builder: PortalFlowsFlowSubscriptionCancelBuilder,
    }

    impl Visitor for Place<PortalFlowsFlowSubscriptionCancel> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsFlowSubscriptionCancelBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsFlowSubscriptionCancelBuilder {
        type Out = PortalFlowsFlowSubscriptionCancel;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "retention" => Deserialize::begin(&mut self.retention),
                "subscription" => Deserialize::begin(&mut self.subscription),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { retention: Deserialize::default(), subscription: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(retention), Some(subscription)) =
                (self.retention.take(), self.subscription.take())
            else {
                return None;
            };
            Some(Self::Out { retention, subscription })
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

    impl ObjectDeser for PortalFlowsFlowSubscriptionCancel {
        type Builder = PortalFlowsFlowSubscriptionCancelBuilder;
    }

    impl FromValueOpt for PortalFlowsFlowSubscriptionCancel {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsFlowSubscriptionCancelBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "retention" => b.retention = FromValueOpt::from_value(v),
                    "subscription" => b.subscription = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
