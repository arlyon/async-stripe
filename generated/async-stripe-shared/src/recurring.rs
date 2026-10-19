#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Recurring {
    /// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    /// The meter tracking the usage of a metered price
    pub meter: Option<String>,
    /// Default number of trial days when subscribing a customer to this price using [`trial_from_plan=true`](https://docs.stripe.com/api#create_subscription-trial_from_plan).
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined.
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub usage_type: stripe_shared::UsageType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Recurring {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Recurring").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RecurringBuilder {
    interval: Option<stripe_shared::PlanInterval>,
    interval_count: Option<u64>,
    meter: Option<Option<String>>,
    trial_period_days: Option<Option<u32>>,
    usage_type: Option<stripe_shared::UsageType>,
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

    impl Deserialize for Recurring {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Recurring>,
        builder: RecurringBuilder,
    }

    impl Visitor for Place<Recurring> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RecurringBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RecurringBuilder {
        type Out = Recurring;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "interval" => Deserialize::begin(&mut self.interval),
                "interval_count" => Deserialize::begin(&mut self.interval_count),
                "meter" => Deserialize::begin(&mut self.meter),
                "trial_period_days" => Deserialize::begin(&mut self.trial_period_days),
                "usage_type" => Deserialize::begin(&mut self.usage_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                interval: None,
                interval_count: None,
                meter: Some(None),
                trial_period_days: Some(None),
                usage_type: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(interval),
                Some(interval_count),
                Some(meter),
                Some(trial_period_days),
                Some(usage_type),
            ) = (
                self.interval.take(),
                self.interval_count,
                self.meter.take(),
                self.trial_period_days,
                self.usage_type.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { interval, interval_count, meter, trial_period_days, usage_type })
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

    impl ObjectDeser for Recurring {
        type Builder = RecurringBuilder;
    }

    impl FromValueOpt for Recurring {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RecurringBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "interval" => b.interval = FromValueOpt::from_value(v),
                    "interval_count" => b.interval_count = FromValueOpt::from_value(v),
                    "meter" => b.meter = FromValueOpt::from_value(v),
                    "trial_period_days" => b.trial_period_days = FromValueOpt::from_value(v),
                    "usage_type" => b.usage_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
pub type RecurringInterval = stripe_shared::PlanInterval;
/// Configures how the quantity per period should be determined.
/// Can be either `metered` or `licensed`.
/// `licensed` automatically bills the `quantity` set when adding it to a subscription.
/// `metered` aggregates the total usage based on usage records.
/// Defaults to `licensed`.
pub type RecurringUsageType = stripe_shared::UsageType;
