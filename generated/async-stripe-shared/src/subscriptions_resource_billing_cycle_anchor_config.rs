#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceBillingCycleAnchorConfig {
    /// The day of the month of the billing_cycle_anchor.
    pub day_of_month: i64,
    /// The hour of the day of the billing_cycle_anchor.
    pub hour: Option<i64>,
    /// The minute of the hour of the billing_cycle_anchor.
    pub minute: Option<i64>,
    /// The month to start full cycle billing periods.
    pub month: Option<i64>,
    /// The second of the minute of the billing_cycle_anchor.
    pub second: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingCycleAnchorConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceBillingCycleAnchorConfig").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceBillingCycleAnchorConfigBuilder {
    day_of_month: Option<i64>,
    hour: Option<Option<i64>>,
    minute: Option<Option<i64>>,
    month: Option<Option<i64>>,
    second: Option<Option<i64>>,
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

    impl Deserialize for SubscriptionsResourceBillingCycleAnchorConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceBillingCycleAnchorConfig>,
        builder: SubscriptionsResourceBillingCycleAnchorConfigBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceBillingCycleAnchorConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceBillingCycleAnchorConfigBuilder {
                    day_of_month: Deserialize::default(),
                    hour: Deserialize::default(),
                    minute: Deserialize::default(),
                    month: Deserialize::default(),
                    second: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "day_of_month" => Deserialize::begin(&mut self.builder.day_of_month),
                "hour" => Deserialize::begin(&mut self.builder.hour),
                "minute" => Deserialize::begin(&mut self.builder.minute),
                "month" => Deserialize::begin(&mut self.builder.month),
                "second" => Deserialize::begin(&mut self.builder.second),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(day_of_month), Some(hour), Some(minute), Some(month), Some(second)) = (
                self.builder.day_of_month,
                self.builder.hour,
                self.builder.minute,
                self.builder.month,
                self.builder.second,
            ) else {
                return Ok(());
            };
            *self.out = Some(SubscriptionsResourceBillingCycleAnchorConfig {
                day_of_month,
                hour,
                minute,
                month,
                second,
            });
            Ok(())
        }
    }
};
