#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourceSettlementTiming {
    /// The number of days charge funds are held before becoming available.
    pub delay_days: u32,
    /// The number of days charge funds are held before becoming available.
    /// If present, overrides the default, or minimum available, for the account.
    pub delay_days_override: Option<u32>,
    /// Customized start of day configuration for automatic payouts to group and send payments in local timezones with a customized day starting time.
    /// For details, see our [Customized start of day](/connect/customized-start-of-day) documentation.
    pub start_of_day: Option<stripe_core::BalanceSettingsResourceStartOfDay>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettingsResourceSettlementTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceSettingsResourceSettlementTiming").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceSettingsResourceSettlementTimingBuilder {
    delay_days: Option<u32>,
    delay_days_override: Option<Option<u32>>,
    start_of_day: Option<Option<stripe_core::BalanceSettingsResourceStartOfDay>>,
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

    impl Deserialize for BalanceSettingsResourceSettlementTiming {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceSettingsResourceSettlementTiming>,
        builder: BalanceSettingsResourceSettlementTimingBuilder,
    }

    impl Visitor for Place<BalanceSettingsResourceSettlementTiming> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceSettingsResourceSettlementTimingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceSettingsResourceSettlementTimingBuilder {
        type Out = BalanceSettingsResourceSettlementTiming;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "delay_days" => Deserialize::begin(&mut self.delay_days),
                "delay_days_override" => Deserialize::begin(&mut self.delay_days_override),
                "start_of_day" => Deserialize::begin(&mut self.start_of_day),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { delay_days: None, delay_days_override: Some(None), start_of_day: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(delay_days), Some(delay_days_override), Some(start_of_day)) =
                (self.delay_days, self.delay_days_override, self.start_of_day.take())
            else {
                return None;
            };
            Some(Self::Out { delay_days, delay_days_override, start_of_day })
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

    impl ObjectDeser for BalanceSettingsResourceSettlementTiming {
        type Builder = BalanceSettingsResourceSettlementTimingBuilder;
    }

    impl FromValueOpt for BalanceSettingsResourceSettlementTiming {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceSettingsResourceSettlementTimingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "delay_days" => b.delay_days = FromValueOpt::from_value(v),
                    "delay_days_override" => b.delay_days_override = FromValueOpt::from_value(v),
                    "start_of_day" => b.start_of_day = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
