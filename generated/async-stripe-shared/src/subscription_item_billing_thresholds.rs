#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionItemBillingThresholds {
    /// Usage threshold that triggers the subscription to create an invoice
    pub usage_gte: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionItemBillingThresholds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionItemBillingThresholds").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionItemBillingThresholdsBuilder {
    usage_gte: Option<Option<i64>>,
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

    impl Deserialize for SubscriptionItemBillingThresholds {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionItemBillingThresholds>,
        builder: SubscriptionItemBillingThresholdsBuilder,
    }

    impl Visitor for Place<SubscriptionItemBillingThresholds> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionItemBillingThresholdsBuilder {
                    usage_gte: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "usage_gte" => Deserialize::begin(&mut self.builder.usage_gte),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(usage_gte),) = (self.builder.usage_gte,) else {
                return Ok(());
            };
            *self.out = Some(SubscriptionItemBillingThresholds { usage_gte });
            Ok(())
        }
    }
};
