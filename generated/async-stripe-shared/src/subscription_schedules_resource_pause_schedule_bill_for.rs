#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourcePauseScheduleBillFor {
    pub outstanding_usage_through:
        stripe_shared::SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThrough,
    pub unused_time_from: stripe_shared::SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleBillFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourcePauseScheduleBillFor").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourcePauseScheduleBillForBuilder {
    outstanding_usage_through:
        Option<stripe_shared::SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThrough>,
    unused_time_from:
        Option<stripe_shared::SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom>,
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

    impl Deserialize for SubscriptionSchedulesResourcePauseScheduleBillFor {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourcePauseScheduleBillFor>,
        builder: SubscriptionSchedulesResourcePauseScheduleBillForBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourcePauseScheduleBillFor> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionSchedulesResourcePauseScheduleBillForBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourcePauseScheduleBillForBuilder {
        type Out = SubscriptionSchedulesResourcePauseScheduleBillFor;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "outstanding_usage_through" => {
                    Deserialize::begin(&mut self.outstanding_usage_through)
                }
                "unused_time_from" => Deserialize::begin(&mut self.unused_time_from),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { outstanding_usage_through: None, unused_time_from: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(outstanding_usage_through), Some(unused_time_from)) =
                (self.outstanding_usage_through.take(), self.unused_time_from.take())
            else {
                return None;
            };
            Some(Self::Out { outstanding_usage_through, unused_time_from })
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

    impl ObjectDeser for SubscriptionSchedulesResourcePauseScheduleBillFor {
        type Builder = SubscriptionSchedulesResourcePauseScheduleBillForBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourcePauseScheduleBillFor {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionSchedulesResourcePauseScheduleBillForBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "outstanding_usage_through" => {
                        b.outstanding_usage_through = FromValueOpt::from_value(v)
                    }
                    "unused_time_from" => b.unused_time_from = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
