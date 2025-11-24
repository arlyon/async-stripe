#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Recurring {
    /// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: RecurringInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    /// The meter tracking the usage of a metered price
    pub meter: Option<String>,
    /// Default number of trial days when subscribing a customer to this price using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined.
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub usage_type: RecurringUsageType,
}
#[doc(hidden)]
pub struct RecurringBuilder {
    interval: Option<RecurringInterval>,
    interval_count: Option<u64>,
    meter: Option<Option<String>>,
    trial_period_days: Option<Option<u32>>,
    usage_type: Option<RecurringUsageType>,
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
                interval: Deserialize::default(),
                interval_count: Deserialize::default(),
                meter: Deserialize::default(),
                trial_period_days: Deserialize::default(),
                usage_type: Deserialize::default(),
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
            *self.out = self.builder.take_out();
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RecurringInterval {
    pub fn as_str(&self) -> &str {
        use RecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RecurringInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "RecurringInterval");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for RecurringInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RecurringInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RecurringInterval::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(RecurringInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Configures how the quantity per period should be determined.
/// Can be either `metered` or `licensed`.
/// `licensed` automatically bills the `quantity` set when adding it to a subscription.
/// `metered` aggregates the total usage based on usage records.
/// Defaults to `licensed`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RecurringUsageType {
    Licensed,
    Metered,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RecurringUsageType {
    pub fn as_str(&self) -> &str {
        use RecurringUsageType::*;
        match self {
            Licensed => "licensed",
            Metered => "metered",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RecurringUsageType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RecurringUsageType::*;
        match s {
            "licensed" => Ok(Licensed),
            "metered" => Ok(Metered),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "RecurringUsageType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RecurringUsageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for RecurringUsageType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RecurringUsageType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RecurringUsageType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(RecurringUsageType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RecurringUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
