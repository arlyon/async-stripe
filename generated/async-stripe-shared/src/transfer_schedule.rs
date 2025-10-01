#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TransferSchedule {
    /// The number of days charges for the account will be held before being paid out.
    pub delay_days: u32,
    /// How frequently funds will be paid out.
    /// One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
    pub interval: String,
    /// The day of the month funds will be paid out.
    /// Only shown if `interval` is monthly.
    /// Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    pub monthly_anchor: Option<u8>,
    /// The days of the month funds will be paid out.
    /// Only shown if `interval` is monthly.
    /// Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    pub monthly_payout_days: Option<Vec<u32>>,
    /// The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc.
    /// Only shown if `interval` is weekly.
    pub weekly_anchor: Option<String>,
    /// The days of the week when available funds are paid out, specified as an array, for example, [`monday`, `tuesday`].
    /// Only shown if `interval` is weekly.
    pub weekly_payout_days: Option<Vec<TransferScheduleWeeklyPayoutDays>>,
}
#[doc(hidden)]
pub struct TransferScheduleBuilder {
    delay_days: Option<u32>,
    interval: Option<String>,
    monthly_anchor: Option<Option<u8>>,
    monthly_payout_days: Option<Option<Vec<u32>>>,
    weekly_anchor: Option<Option<String>>,
    weekly_payout_days: Option<Option<Vec<TransferScheduleWeeklyPayoutDays>>>,
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

    impl Deserialize for TransferSchedule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TransferSchedule>,
        builder: TransferScheduleBuilder,
    }

    impl Visitor for Place<TransferSchedule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TransferScheduleBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TransferScheduleBuilder {
        type Out = TransferSchedule;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "delay_days" => Deserialize::begin(&mut self.delay_days),
                "interval" => Deserialize::begin(&mut self.interval),
                "monthly_anchor" => Deserialize::begin(&mut self.monthly_anchor),
                "monthly_payout_days" => Deserialize::begin(&mut self.monthly_payout_days),
                "weekly_anchor" => Deserialize::begin(&mut self.weekly_anchor),
                "weekly_payout_days" => Deserialize::begin(&mut self.weekly_payout_days),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                delay_days: Deserialize::default(),
                interval: Deserialize::default(),
                monthly_anchor: Deserialize::default(),
                monthly_payout_days: Deserialize::default(),
                weekly_anchor: Deserialize::default(),
                weekly_payout_days: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(delay_days),
                Some(interval),
                Some(monthly_anchor),
                Some(monthly_payout_days),
                Some(weekly_anchor),
                Some(weekly_payout_days),
            ) = (
                self.delay_days,
                self.interval.take(),
                self.monthly_anchor,
                self.monthly_payout_days.take(),
                self.weekly_anchor.take(),
                self.weekly_payout_days.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                delay_days,
                interval,
                monthly_anchor,
                monthly_payout_days,
                weekly_anchor,
                weekly_payout_days,
            })
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

    impl ObjectDeser for TransferSchedule {
        type Builder = TransferScheduleBuilder;
    }

    impl FromValueOpt for TransferSchedule {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TransferScheduleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "delay_days" => b.delay_days = FromValueOpt::from_value(v),
                    "interval" => b.interval = FromValueOpt::from_value(v),
                    "monthly_anchor" => b.monthly_anchor = FromValueOpt::from_value(v),
                    "monthly_payout_days" => b.monthly_payout_days = FromValueOpt::from_value(v),
                    "weekly_anchor" => b.weekly_anchor = FromValueOpt::from_value(v),
                    "weekly_payout_days" => b.weekly_payout_days = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The days of the week when available funds are paid out, specified as an array, for example, [`monday`, `tuesday`].
/// Only shown if `interval` is weekly.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TransferScheduleWeeklyPayoutDays {
    Friday,
    Monday,
    Thursday,
    Tuesday,
    Wednesday,
}
impl TransferScheduleWeeklyPayoutDays {
    pub fn as_str(self) -> &'static str {
        use TransferScheduleWeeklyPayoutDays::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
        }
    }
}

impl std::str::FromStr for TransferScheduleWeeklyPayoutDays {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransferScheduleWeeklyPayoutDays::*;
        match s {
            "friday" => Ok(Friday),
            "monday" => Ok(Monday),
            "thursday" => Ok(Thursday),
            "tuesday" => Ok(Tuesday),
            "wednesday" => Ok(Wednesday),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TransferScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TransferScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TransferScheduleWeeklyPayoutDays {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TransferScheduleWeeklyPayoutDays {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TransferScheduleWeeklyPayoutDays> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TransferScheduleWeeklyPayoutDays::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TransferScheduleWeeklyPayoutDays);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TransferScheduleWeeklyPayoutDays {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TransferScheduleWeeklyPayoutDays")
        })
    }
}
