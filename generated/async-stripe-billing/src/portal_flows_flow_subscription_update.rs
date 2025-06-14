#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsFlowSubscriptionUpdate {
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
#[doc(hidden)]
pub struct PortalFlowsFlowSubscriptionUpdateBuilder {
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsFlowSubscriptionUpdate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlowSubscriptionUpdate>,
        builder: PortalFlowsFlowSubscriptionUpdateBuilder,
    }

    impl Visitor for Place<PortalFlowsFlowSubscriptionUpdate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsFlowSubscriptionUpdateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsFlowSubscriptionUpdateBuilder {
        type Out = PortalFlowsFlowSubscriptionUpdate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "subscription" => Deserialize::begin(&mut self.subscription),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { subscription: Deserialize::default() }
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
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PortalFlowsFlowSubscriptionUpdate {
        type Builder = PortalFlowsFlowSubscriptionUpdateBuilder;
    }

    impl FromValueOpt for PortalFlowsFlowSubscriptionUpdate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsFlowSubscriptionUpdateBuilder::deser_default();
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
