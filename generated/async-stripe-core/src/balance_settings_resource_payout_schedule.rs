#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourcePayoutSchedule {
    /// How frequently funds will be paid out.
    /// One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
    pub interval: Option<stripe_shared::Interval>,
    /// The day of the month funds will be paid out.
    /// Only shown if `interval` is monthly.
    /// Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    pub monthly_payout_days: Option<Vec<u32>>,
    /// The days of the week when available funds are paid out, specified as an array, for example, [`monday`, `tuesday`].
    /// Only shown if `interval` is weekly.
    pub weekly_payout_days: Option<Vec<stripe_shared::WeeklyPayoutDays>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettingsResourcePayoutSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceSettingsResourcePayoutSchedule").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceSettingsResourcePayoutScheduleBuilder {
    interval: Option<Option<stripe_shared::Interval>>,
    monthly_payout_days: Option<Option<Vec<u32>>>,
    weekly_payout_days: Option<Option<Vec<stripe_shared::WeeklyPayoutDays>>>,
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
                interval: Some(None),
                monthly_payout_days: Some(None),
                weekly_payout_days: Some(None),
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
pub type BalanceSettingsResourcePayoutScheduleInterval = stripe_shared::Interval;
/// The days of the week when available funds are paid out, specified as an array, for example, [`monday`, `tuesday`].
/// Only shown if `interval` is weekly.
pub type BalanceSettingsResourcePayoutScheduleWeeklyPayoutDays = stripe_shared::WeeklyPayoutDays;
