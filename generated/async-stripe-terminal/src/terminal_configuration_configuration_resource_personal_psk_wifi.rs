#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourcePersonalPskWifi {
    /// Password for connecting to the WiFi network
    pub password: String,
    /// Name of the WiFi network
    pub ssid: String,
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourcePersonalPskWifiBuilder {
    password: Option<String>,
    ssid: Option<String>,
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

    impl Deserialize for TerminalConfigurationConfigurationResourcePersonalPskWifi {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourcePersonalPskWifi>,
        builder: TerminalConfigurationConfigurationResourcePersonalPskWifiBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourcePersonalPskWifi> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TerminalConfigurationConfigurationResourcePersonalPskWifiBuilder::deser_default(
                    ),
            }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourcePersonalPskWifiBuilder {
        type Out = TerminalConfigurationConfigurationResourcePersonalPskWifi;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "password" => Deserialize::begin(&mut self.password),
                "ssid" => Deserialize::begin(&mut self.ssid),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { password: Deserialize::default(), ssid: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(password), Some(ssid)) = (self.password.take(), self.ssid.take()) else {
                return None;
            };
            Some(Self::Out { password, ssid })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourcePersonalPskWifi {
        type Builder = TerminalConfigurationConfigurationResourcePersonalPskWifiBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourcePersonalPskWifi {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TerminalConfigurationConfigurationResourcePersonalPskWifiBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "password" => b.password = FromValueOpt::from_value(v),
                    "ssid" => b.ssid = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
