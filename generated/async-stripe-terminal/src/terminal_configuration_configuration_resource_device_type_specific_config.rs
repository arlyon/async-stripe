#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    /// A File ID representing an image you would like displayed on the reader.
    pub splashscreen: Option<stripe_types::Expandable<stripe_shared::File>>,
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder {
    splashscreen: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            builder: TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder {
        type Out = TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "splashscreen" => Deserialize::begin(&mut self.splashscreen),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { splashscreen: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { splashscreen: self.splashscreen.take()? })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
        type Builder = TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "splashscreen" => b.splashscreen = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
