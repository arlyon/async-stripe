#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourceSettlementTiming {
    /// The number of days charge funds are held before becoming available.
    pub delay_days: u32,
    /// The number of days charge funds are held before becoming available.
    /// If present, overrides the default, or minimum available, for the account.
    pub delay_days_override: Option<u32>,
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
                builder: BalanceSettingsResourceSettlementTimingBuilder {
                    delay_days: Deserialize::default(),
                    delay_days_override: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "delay_days" => Deserialize::begin(&mut self.builder.delay_days),
                "delay_days_override" => Deserialize::begin(&mut self.builder.delay_days_override),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(delay_days), Some(delay_days_override)) =
                (self.builder.delay_days, self.builder.delay_days_override)
            else {
                return Ok(());
            };
            *self.out =
                Some(BalanceSettingsResourceSettlementTiming { delay_days, delay_days_override });
            Ok(())
        }
    }
};
