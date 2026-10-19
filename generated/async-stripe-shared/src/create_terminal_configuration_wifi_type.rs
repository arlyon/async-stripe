#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTerminalConfigurationWifiType {
    EnterpriseEapPeap,
    EnterpriseEapTls,
    PersonalPsk,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTerminalConfigurationWifiType {
    pub fn as_str(&self) -> &str {
        use CreateTerminalConfigurationWifiType::*;
        match self {
            EnterpriseEapPeap => "enterprise_eap_peap",
            EnterpriseEapTls => "enterprise_eap_tls",
            PersonalPsk => "personal_psk",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTerminalConfigurationWifiType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTerminalConfigurationWifiType::*;
        match s {
            "enterprise_eap_peap" => Ok(EnterpriseEapPeap),
            "enterprise_eap_tls" => Ok(EnterpriseEapTls),
            "personal_psk" => Ok(PersonalPsk),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTerminalConfigurationWifiType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTerminalConfigurationWifiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateTerminalConfigurationWifiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTerminalConfigurationWifiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateTerminalConfigurationWifiType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateTerminalConfigurationWifiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreateTerminalConfigurationWifiType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreateTerminalConfigurationWifiType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreateTerminalConfigurationWifiType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreateTerminalConfigurationWifiType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTerminalConfigurationWifiType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
