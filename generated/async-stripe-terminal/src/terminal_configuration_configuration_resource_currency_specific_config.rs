#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    /// Fixed amounts displayed when collecting a tip
    pub fixed_amounts: Option<Vec<i64>>,
    /// Percentages displayed when collecting a tip
    pub percentages: Option<Vec<i64>>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    pub smart_tip_threshold: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfigurationConfigurationResourceCurrencySpecificConfig")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder {
    fixed_amounts: Option<Option<Vec<i64>>>,
    percentages: Option<Option<Vec<i64>>>,
    smart_tip_threshold: Option<Option<i64>>,
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

    impl Deserialize for TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
        builder: TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceCurrencySpecificConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder {
                    fixed_amounts: Deserialize::default(),
                    percentages: Deserialize::default(),
                    smart_tip_threshold: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fixed_amounts" => Deserialize::begin(&mut self.builder.fixed_amounts),
                "percentages" => Deserialize::begin(&mut self.builder.percentages),
                "smart_tip_threshold" => Deserialize::begin(&mut self.builder.smart_tip_threshold),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(fixed_amounts), Some(percentages), Some(smart_tip_threshold)) = (
                self.builder.fixed_amounts.take(),
                self.builder.percentages.take(),
                self.builder.smart_tip_threshold,
            ) else {
                return Ok(());
            };
            *self.out = Some(TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
                fixed_amounts,
                percentages,
                smart_tip_threshold,
            });
            Ok(())
        }
    }
};
