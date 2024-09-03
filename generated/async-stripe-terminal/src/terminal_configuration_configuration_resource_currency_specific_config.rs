#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder {
    fixed_amounts: Option<Option<Vec<i64>>>,
    percentages: Option<Option<Vec<i64>>>,
    smart_tip_threshold: Option<Option<i64>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            builder: TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder {
        type Out = TerminalConfigurationConfigurationResourceCurrencySpecificConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fixed_amounts" => Deserialize::begin(&mut self.fixed_amounts),
                "percentages" => Deserialize::begin(&mut self.percentages),
                "smart_tip_threshold" => Deserialize::begin(&mut self.smart_tip_threshold),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                fixed_amounts: Deserialize::default(),
                percentages: Deserialize::default(),
                smart_tip_threshold: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(fixed_amounts), Some(percentages), Some(smart_tip_threshold)) =
                (self.fixed_amounts.take(), self.percentages.take(), self.smart_tip_threshold)
            else {
                return None;
            };
            Some(Self::Out { fixed_amounts, percentages, smart_tip_threshold })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
        type Builder = TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fixed_amounts" => b.fixed_amounts = FromValueOpt::from_value(v),
                    "percentages" => b.percentages = FromValueOpt::from_value(v),
                    "smart_tip_threshold" => b.smart_tip_threshold = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
