#[derive(Clone, Debug)]
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
                builder: TerminalConfigurationConfigurationResourceWifiConfigBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceWifiConfigBuilder {
        type Out = TerminalConfigurationConfigurationResourceWifiConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enterprise_eap_peap" => Deserialize::begin(&mut self.enterprise_eap_peap),
                "enterprise_eap_tls" => Deserialize::begin(&mut self.enterprise_eap_tls),
                "personal_psk" => Deserialize::begin(&mut self.personal_psk),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                enterprise_eap_peap: Deserialize::default(),
                enterprise_eap_tls: Deserialize::default(),
                personal_psk: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(enterprise_eap_peap),
                Some(enterprise_eap_tls),
                Some(personal_psk),
                Some(type_),
            ) = (
                self.enterprise_eap_peap.take(),
                self.enterprise_eap_tls.take(),
                self.personal_psk.take(),
                self.type_,
            )
            else {
                return None;
            };
            Some(Self::Out { enterprise_eap_peap, enterprise_eap_tls, personal_psk, type_ })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceWifiConfig {
        type Builder = TerminalConfigurationConfigurationResourceWifiConfigBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceWifiConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TerminalConfigurationConfigurationResourceWifiConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enterprise_eap_peap" => b.enterprise_eap_peap = FromValueOpt::from_value(v),
                    "enterprise_eap_tls" => b.enterprise_eap_tls = FromValueOpt::from_value(v),
                    "personal_psk" => b.personal_psk = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Security type of the WiFi network.
/// The hash with the corresponding name contains the credentials for this security type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalConfigurationConfigurationResourceWifiConfigType {
    EnterpriseEapPeap,
    EnterpriseEapTls,
    PersonalPsk,
}
impl TerminalConfigurationConfigurationResourceWifiConfigType {
    pub fn as_str(self) -> &'static str {
        use TerminalConfigurationConfigurationResourceWifiConfigType::*;
        match self {
            EnterpriseEapPeap => "enterprise_eap_peap",
            EnterpriseEapTls => "enterprise_eap_tls",
            PersonalPsk => "personal_psk",
        }
    }
}

impl std::str::FromStr for TerminalConfigurationConfigurationResourceWifiConfigType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalConfigurationConfigurationResourceWifiConfigType::*;
        match s {
            "enterprise_eap_peap" => Ok(EnterpriseEapPeap),
            "enterprise_eap_tls" => Ok(EnterpriseEapTls),
            "personal_psk" => Ok(PersonalPsk),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TerminalConfigurationConfigurationResourceWifiConfigType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TerminalConfigurationConfigurationResourceWifiConfigType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TerminalConfigurationConfigurationResourceWifiConfigType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TerminalConfigurationConfigurationResourceWifiConfigType",
            )
        })
    }
}
