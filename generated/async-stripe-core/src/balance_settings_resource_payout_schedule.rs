#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourcePayoutSchedule {
    /// How frequently funds will be paid out.
    /// One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
    pub interval: Option<BalanceSettingsResourcePayoutScheduleInterval>,
    /// The day of the month funds will be paid out.
    /// Only shown if `interval` is monthly.
    /// Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    pub monthly_payout_days: Option<Vec<u32>>,
    /// The days of the week when available funds are paid out, specified as an array, for example, [`monday`, `tuesday`].
    /// Only shown if `interval` is weekly.
    pub weekly_payout_days: Option<Vec<BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays>>,
}
#[doc(hidden)]
pub struct BalanceSettingsResourcePayoutScheduleBuilder {
    interval: Option<Option<BalanceSettingsResourcePayoutScheduleInterval>>,
    monthly_payout_days: Option<Option<Vec<u32>>>,
    weekly_payout_days: Option<Option<Vec<BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays>>>,
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

    impl Deserialize for BalanceSettingsResourcePayoutSchedule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceSettingsResourcePayoutSchedule>,
        builder: BalanceSettingsResourcePayoutScheduleBuilder,
    }

    impl Visitor for Place<BalanceSettingsResourcePayoutSchedule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceSettingsResourcePayoutScheduleBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceSettingsResourcePayoutScheduleBuilder {
        type Out = BalanceSettingsResourcePayoutSchedule;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "interval" => Deserialize::begin(&mut self.interval),
                "monthly_payout_days" => Deserialize::begin(&mut self.monthly_payout_days),
                "weekly_payout_days" => Deserialize::begin(&mut self.weekly_payout_days),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                interval: Deserialize::default(),
                monthly_payout_days: Deserialize::default(),
                weekly_payout_days: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(interval), Some(monthly_payout_days), Some(weekly_payout_days)) = (
                self.interval.take(),
                self.monthly_payout_days.take(),
                self.weekly_payout_days.take(),
            ) else {
                return None;
            };
            Some(Self::Out { interval, monthly_payout_days, weekly_payout_days })
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

    impl ObjectDeser for BalanceSettingsResourcePayoutSchedule {
        type Builder = BalanceSettingsResourcePayoutScheduleBuilder;
    }

    impl FromValueOpt for BalanceSettingsResourcePayoutSchedule {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceSettingsResourcePayoutScheduleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "interval" => b.interval = FromValueOpt::from_value(v),
                    "monthly_payout_days" => b.monthly_payout_days = FromValueOpt::from_value(v),
                    "weekly_payout_days" => b.weekly_payout_days = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// How frequently funds will be paid out.
/// One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BalanceSettingsResourcePayoutScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BalanceSettingsResourcePayoutScheduleInterval {
    pub fn as_str(&self) -> &str {
        use BalanceSettingsResourcePayoutScheduleInterval::*;
        match self {
            Daily => "daily",
            Manual => "manual",
            Monthly => "monthly",
            Weekly => "weekly",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BalanceSettingsResourcePayoutScheduleInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BalanceSettingsResourcePayoutScheduleInterval::*;
        match s {
            "daily" => Ok(Daily),
            "manual" => Ok(Manual),
            "monthly" => Ok(Monthly),
            "weekly" => Ok(Weekly),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BalanceSettingsResourcePayoutScheduleInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BalanceSettingsResourcePayoutScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BalanceSettingsResourcePayoutScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BalanceSettingsResourcePayoutScheduleInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BalanceSettingsResourcePayoutScheduleInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BalanceSettingsResourcePayoutScheduleInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BalanceSettingsResourcePayoutScheduleInterval::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BalanceSettingsResourcePayoutScheduleInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BalanceSettingsResourcePayoutScheduleInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The days of the week when available funds are paid out, specified as an array, for example, [`monday`, `tuesday`].
/// Only shown if `interval` is weekly.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays {
    Friday,
    Monday,
    Thursday,
    Tuesday,
    Wednesday,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays {
    pub fn as_str(&self) -> &str {
        use BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays::*;
        match s {
            "friday" => Ok(Friday),
            "monday" => Ok(Monday),
            "thursday" => Ok(Thursday),
            "tuesday" => Ok(Tuesday),
            "wednesday" => Ok(Wednesday),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
