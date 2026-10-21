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
                day_of_month: None,
                hour: Some(None),
                minute: Some(None),
                month: Some(None),
                second: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    day_of_month: Some(day_of_month),
                    hour: Some(hour),
                    minute: Some(minute),
                    month: Some(month),
                    second: Some(second),
                } => Some(Self::Out { day_of_month, hour, minute, month, second }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for SubscriptionsResourceBillingCycleAnchorConfig {
        type Builder = SubscriptionsResourceBillingCycleAnchorConfigBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceBillingCycleAnchorConfig {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
