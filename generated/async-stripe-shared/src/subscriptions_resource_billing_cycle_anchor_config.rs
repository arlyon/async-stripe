#[derive(Copy, Clone, Debug)]
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
                builder: SubscriptionsResourceBillingCycleAnchorConfigBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceBillingCycleAnchorConfigBuilder {
        type Out = SubscriptionsResourceBillingCycleAnchorConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "day_of_month" => Deserialize::begin(&mut self.day_of_month),
                "hour" => Deserialize::begin(&mut self.hour),
                "minute" => Deserialize::begin(&mut self.minute),
                "month" => Deserialize::begin(&mut self.month),
                "second" => Deserialize::begin(&mut self.second),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                day_of_month: Deserialize::default(),
                hour: Deserialize::default(),
                minute: Deserialize::default(),
                month: Deserialize::default(),
                second: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(day_of_month), Some(hour), Some(minute), Some(month), Some(second)) =
                (self.day_of_month, self.hour, self.minute, self.month, self.second)
            else {
                return None;
            };
            Some(Self::Out { day_of_month, hour, minute, month, second })
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

    impl ObjectDeser for SubscriptionsResourceBillingCycleAnchorConfig {
        type Builder = SubscriptionsResourceBillingCycleAnchorConfigBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceBillingCycleAnchorConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourceBillingCycleAnchorConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "day_of_month" => b.day_of_month = FromValueOpt::from_value(v),
                    "hour" => b.hour = FromValueOpt::from_value(v),
                    "minute" => b.minute = FromValueOpt::from_value(v),
                    "month" => b.month = FromValueOpt::from_value(v),
                    "second" => b.second = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
