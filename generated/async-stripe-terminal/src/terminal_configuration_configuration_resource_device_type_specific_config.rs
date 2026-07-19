#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    /// A File ID representing an image to display on the reader
    pub splashscreen: Option<stripe_types::Expandable<stripe_shared::File>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder {
    splashscreen: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
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

    impl Deserialize for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
        builder: TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder {
                        splashscreen: Deserialize::default(),
                    },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "splashscreen" => Deserialize::begin(&mut self.builder.splashscreen),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(splashscreen),) = (self.builder.splashscreen.take(),) else {
                return Ok(());
            };
            *self.out = Some(TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
                splashscreen,
            });
            Ok(())
        }
    }
};
