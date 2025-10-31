#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceEnterpriseTlsWifi {
    /// A File ID representing a PEM file containing the server certificate
    pub ca_certificate_file: Option<String>,
    /// A File ID representing a PEM file containing the client certificate
    pub client_certificate_file: String,
    /// A File ID representing a PEM file containing the client RSA private key
    pub private_key_file: String,
    /// Password for the private key file
    pub private_key_file_password: Option<String>,
    /// Name of the WiFi network
    pub ssid: String,
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceEnterpriseTlsWifiBuilder {
    ca_certificate_file: Option<Option<String>>,
    client_certificate_file: Option<String>,
    private_key_file: Option<String>,
    private_key_file_password: Option<Option<String>>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalConfigurationConfigurationResourceEnterpriseTlsWifi {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceEnterpriseTlsWifi>,
        builder: TerminalConfigurationConfigurationResourceEnterpriseTlsWifiBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceEnterpriseTlsWifi> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TerminalConfigurationConfigurationResourceEnterpriseTlsWifiBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceEnterpriseTlsWifiBuilder {
        type Out = TerminalConfigurationConfigurationResourceEnterpriseTlsWifi;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ca_certificate_file" => Deserialize::begin(&mut self.ca_certificate_file),
                "client_certificate_file" => Deserialize::begin(&mut self.client_certificate_file),
                "private_key_file" => Deserialize::begin(&mut self.private_key_file),
                "private_key_file_password" => {
                    Deserialize::begin(&mut self.private_key_file_password)
                }
                "ssid" => Deserialize::begin(&mut self.ssid),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                ca_certificate_file: Deserialize::default(),
                client_certificate_file: Deserialize::default(),
                private_key_file: Deserialize::default(),
                private_key_file_password: Deserialize::default(),
                ssid: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(ca_certificate_file),
                Some(client_certificate_file),
                Some(private_key_file),
                Some(private_key_file_password),
                Some(ssid),
            ) = (
                self.ca_certificate_file.take(),
                self.client_certificate_file.take(),
                self.private_key_file.take(),
                self.private_key_file_password.take(),
                self.ssid.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                ca_certificate_file,
                client_certificate_file,
                private_key_file,
                private_key_file_password,
                ssid,
            })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceEnterpriseTlsWifi {
        type Builder = TerminalConfigurationConfigurationResourceEnterpriseTlsWifiBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceEnterpriseTlsWifi {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TerminalConfigurationConfigurationResourceEnterpriseTlsWifiBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ca_certificate_file" => b.ca_certificate_file = FromValueOpt::from_value(v),
                    "client_certificate_file" => {
                        b.client_certificate_file = FromValueOpt::from_value(v)
                    }
                    "private_key_file" => b.private_key_file = FromValueOpt::from_value(v),
                    "private_key_file_password" => {
                        b.private_key_file_password = FromValueOpt::from_value(v)
                    }
                    "ssid" => b.ssid = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
