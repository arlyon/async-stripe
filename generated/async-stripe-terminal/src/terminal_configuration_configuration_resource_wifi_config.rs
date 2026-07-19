#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceWifiConfig {
    pub enterprise_eap_peap:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceEnterprisePeapWifi>,
    pub enterprise_eap_tls:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceEnterpriseTlsWifi>,
    pub personal_psk:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourcePersonalPskWifi>,
    /// Security type of the WiFi network.
    /// The hash with the corresponding name contains the credentials for this security type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TerminalConfigurationConfigurationResourceWifiConfigType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceWifiConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfigurationConfigurationResourceWifiConfig")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceWifiConfigBuilder {
    enterprise_eap_peap: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceEnterprisePeapWifi>,
    >,
    enterprise_eap_tls: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceEnterpriseTlsWifi>,
    >,
    personal_psk:
        Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourcePersonalPskWifi>>,
    type_: Option<TerminalConfigurationConfigurationResourceWifiConfigType>,
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

    impl Deserialize for TerminalConfigurationConfigurationResourceWifiConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceWifiConfig>,
        builder: TerminalConfigurationConfigurationResourceWifiConfigBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceWifiConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalConfigurationConfigurationResourceWifiConfigBuilder {
                    enterprise_eap_peap: Deserialize::default(),
                    enterprise_eap_tls: Deserialize::default(),
                    personal_psk: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enterprise_eap_peap" => Deserialize::begin(&mut self.builder.enterprise_eap_peap),
                "enterprise_eap_tls" => Deserialize::begin(&mut self.builder.enterprise_eap_tls),
                "personal_psk" => Deserialize::begin(&mut self.builder.personal_psk),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(enterprise_eap_peap),
                Some(enterprise_eap_tls),
                Some(personal_psk),
                Some(type_),
            ) = (
                self.builder.enterprise_eap_peap.take(),
                self.builder.enterprise_eap_tls.take(),
                self.builder.personal_psk.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TerminalConfigurationConfigurationResourceWifiConfig {
                enterprise_eap_peap,
                enterprise_eap_tls,
                personal_psk,
                type_,
            });
            Ok(())
        }
    }
};
/// Security type of the WiFi network.
/// The hash with the corresponding name contains the credentials for this security type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalConfigurationConfigurationResourceWifiConfigType {
    EnterpriseEapPeap,
    EnterpriseEapTls,
    PersonalPsk,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalConfigurationConfigurationResourceWifiConfigType {
    pub fn as_str(&self) -> &str {
        use TerminalConfigurationConfigurationResourceWifiConfigType::*;
        match self {
            EnterpriseEapPeap => "enterprise_eap_peap",
            EnterpriseEapTls => "enterprise_eap_tls",
            PersonalPsk => "personal_psk",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalConfigurationConfigurationResourceWifiConfigType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalConfigurationConfigurationResourceWifiConfigType::*;
        match s {
            "enterprise_eap_peap" => Ok(EnterpriseEapPeap),
            "enterprise_eap_tls" => Ok(EnterpriseEapTls),
            "personal_psk" => Ok(PersonalPsk),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TerminalConfigurationConfigurationResourceWifiConfigType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TerminalConfigurationConfigurationResourceWifiConfigType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TerminalConfigurationConfigurationResourceWifiConfigType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TerminalConfigurationConfigurationResourceWifiConfigType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
