#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsFlowSubscriptionUpdate {
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsFlowSubscriptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFlowsFlowSubscriptionUpdate").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalFlowsFlowSubscriptionUpdateBuilder {
    subscription: Option<String>,
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
                builder: PortalFlowsFlowSubscriptionUpdateBuilder {
                    subscription: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "subscription" => Deserialize::begin(&mut self.builder.subscription),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(subscription),) = (self.builder.subscription.take(),) else {
                return Ok(());
            };
            *self.out = Some(PortalFlowsFlowSubscriptionUpdate { subscription });
            Ok(())
        }
    }
};
