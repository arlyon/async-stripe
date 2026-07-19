#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceEnterprisePeapWifi {
    /// A File ID representing a PEM file containing the server certificate
    pub ca_certificate_file: Option<String>,
    /// Password for connecting to the WiFi network
    pub password: String,
    /// Name of the WiFi network
    pub ssid: String,
    /// Username for connecting to the WiFi network
    pub username: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceEnterprisePeapWifi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfigurationConfigurationResourceEnterprisePeapWifi")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceEnterprisePeapWifiBuilder {
    ca_certificate_file: Option<Option<String>>,
    password: Option<String>,
    ssid: Option<String>,
    username: Option<String>,
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

    impl Deserialize for TerminalConfigurationConfigurationResourceEnterprisePeapWifi {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceEnterprisePeapWifi>,
        builder: TerminalConfigurationConfigurationResourceEnterprisePeapWifiBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceEnterprisePeapWifi> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalConfigurationConfigurationResourceEnterprisePeapWifiBuilder {
                    ca_certificate_file: Deserialize::default(),
                    password: Deserialize::default(),
                    ssid: Deserialize::default(),
                    username: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ca_certificate_file" => Deserialize::begin(&mut self.builder.ca_certificate_file),
                "password" => Deserialize::begin(&mut self.builder.password),
                "ssid" => Deserialize::begin(&mut self.builder.ssid),
                "username" => Deserialize::begin(&mut self.builder.username),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(ca_certificate_file), Some(password), Some(ssid), Some(username)) = (
                self.builder.ca_certificate_file.take(),
                self.builder.password.take(),
                self.builder.ssid.take(),
                self.builder.username.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(TerminalConfigurationConfigurationResourceEnterprisePeapWifi {
                ca_certificate_file,
                password,
                ssid,
                username,
            });
            Ok(())
        }
    }
};
