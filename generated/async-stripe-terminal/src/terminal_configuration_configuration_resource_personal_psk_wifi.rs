#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourcePersonalPskWifi {
    /// Password for connecting to the WiFi network
    pub password: String,
    /// Name of the WiFi network
    pub ssid: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourcePersonalPskWifi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfigurationConfigurationResourcePersonalPskWifi")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourcePersonalPskWifiBuilder {
    password: Option<String>,
    ssid: Option<String>,
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
                builder: TerminalConfigurationConfigurationResourcePersonalPskWifiBuilder {
                    password: Deserialize::default(),
                    ssid: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "password" => Deserialize::begin(&mut self.builder.password),
                "ssid" => Deserialize::begin(&mut self.builder.ssid),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(password), Some(ssid)) =
                (self.builder.password.take(), self.builder.ssid.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(TerminalConfigurationConfigurationResourcePersonalPskWifi { password, ssid });
            Ok(())
        }
    }
};
