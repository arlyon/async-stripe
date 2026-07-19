#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TransferSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TransferSchedule").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TransferScheduleBuilder {
                    delay_days: Deserialize::default(),
                    interval: Deserialize::default(),
                    monthly_anchor: Deserialize::default(),
                    monthly_payout_days: Deserialize::default(),
                    weekly_anchor: Deserialize::default(),
                    weekly_payout_days: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "delay_days" => Deserialize::begin(&mut self.builder.delay_days),
                "interval" => Deserialize::begin(&mut self.builder.interval),
                "monthly_anchor" => Deserialize::begin(&mut self.builder.monthly_anchor),
                "monthly_payout_days" => Deserialize::begin(&mut self.builder.monthly_payout_days),
                "weekly_anchor" => Deserialize::begin(&mut self.builder.weekly_anchor),
                "weekly_payout_days" => Deserialize::begin(&mut self.builder.weekly_payout_days),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(delay_days),
                Some(interval),
                Some(monthly_anchor),
                Some(monthly_payout_days),
                Some(weekly_anchor),
                Some(weekly_payout_days),
            ) = (
                self.builder.delay_days,
                self.builder.interval.take(),
                self.builder.monthly_anchor,
                self.builder.monthly_payout_days.take(),
                self.builder.weekly_anchor.take(),
                self.builder.weekly_payout_days.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TransferSchedule {
                delay_days,
                interval,
                monthly_anchor,
                monthly_payout_days,
                weekly_anchor,
                weekly_payout_days,
            });
            Ok(())
        }
    }
};
/// The days of the week when available funds are paid out, specified as an array, for example, [`monday`, `tuesday`].
/// Only shown if `interval` is weekly.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TransferScheduleWeeklyPayoutDays {
    Friday,
    Monday,
    Thursday,
    Tuesday,
    Wednesday,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TransferScheduleWeeklyPayoutDays {
    pub fn as_str(&self) -> &str {
        use TransferScheduleWeeklyPayoutDays::*;
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

impl std::str::FromStr for TransferScheduleWeeklyPayoutDays {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransferScheduleWeeklyPayoutDays::*;
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
                    "TransferScheduleWeeklyPayoutDays"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TransferScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TransferScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TransferScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TransferScheduleWeeklyPayoutDays)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for TransferScheduleWeeklyPayoutDays {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TransferScheduleWeeklyPayoutDays> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TransferScheduleWeeklyPayoutDays::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TransferScheduleWeeklyPayoutDays {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
