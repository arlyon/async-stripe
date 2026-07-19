#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceEnterpriseTlsWifi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfigurationConfigurationResourceEnterpriseTlsWifi")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TerminalConfigurationConfigurationResourceEnterpriseTlsWifiBuilder {
                    ca_certificate_file: Deserialize::default(),
                    client_certificate_file: Deserialize::default(),
                    private_key_file: Deserialize::default(),
                    private_key_file_password: Deserialize::default(),
                    ssid: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ca_certificate_file" => Deserialize::begin(&mut self.builder.ca_certificate_file),
                "client_certificate_file" => {
                    Deserialize::begin(&mut self.builder.client_certificate_file)
                }
                "private_key_file" => Deserialize::begin(&mut self.builder.private_key_file),
                "private_key_file_password" => {
                    Deserialize::begin(&mut self.builder.private_key_file_password)
                }
                "ssid" => Deserialize::begin(&mut self.builder.ssid),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(ca_certificate_file),
                Some(client_certificate_file),
                Some(private_key_file),
                Some(private_key_file_password),
                Some(ssid),
            ) = (
                self.builder.ca_certificate_file.take(),
                self.builder.client_certificate_file.take(),
                self.builder.private_key_file.take(),
                self.builder.private_key_file_password.take(),
                self.builder.ssid.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TerminalConfigurationConfigurationResourceEnterpriseTlsWifi {
                ca_certificate_file,
                client_certificate_file,
                private_key_file,
                private_key_file_password,
                ssid,
            });
            Ok(())
        }
    }
};
