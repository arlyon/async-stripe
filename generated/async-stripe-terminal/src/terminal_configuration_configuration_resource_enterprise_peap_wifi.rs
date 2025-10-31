#[derive(Clone, Debug)]
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
            builder: TerminalConfigurationConfigurationResourceEnterprisePeapWifiBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceEnterprisePeapWifiBuilder {
        type Out = TerminalConfigurationConfigurationResourceEnterprisePeapWifi;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ca_certificate_file" => Deserialize::begin(&mut self.ca_certificate_file),
                "password" => Deserialize::begin(&mut self.password),
                "ssid" => Deserialize::begin(&mut self.ssid),
                "username" => Deserialize::begin(&mut self.username),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                ca_certificate_file: Deserialize::default(),
                password: Deserialize::default(),
                ssid: Deserialize::default(),
                username: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(ca_certificate_file), Some(password), Some(ssid), Some(username)) = (
                self.ca_certificate_file.take(),
                self.password.take(),
                self.ssid.take(),
                self.username.take(),
            ) else {
                return None;
            };
            Some(Self::Out { ca_certificate_file, password, ssid, username })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceEnterprisePeapWifi {
        type Builder = TerminalConfigurationConfigurationResourceEnterprisePeapWifiBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceEnterprisePeapWifi {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TerminalConfigurationConfigurationResourceEnterprisePeapWifiBuilder::deser_default(
                );
            for (k, v) in obj {
                match k.as_str() {
                    "ca_certificate_file" => b.ca_certificate_file = FromValueOpt::from_value(v),
                    "password" => b.password = FromValueOpt::from_value(v),
                    "ssid" => b.ssid = FromValueOpt::from_value(v),
                    "username" => b.username = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
