/// A Configurations object represents how features should be configured for terminal readers.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalConfigurationConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_configuration_configuration::TerminalConfigurationId,
    /// Whether this Configuration is the default for your account.
    pub is_account_default: Option<bool>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TerminalConfigurationConfigurationObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalConfigurationConfigurationObject {
    TerminalConfiguration,
}

impl TerminalConfigurationConfigurationObject {
    pub fn as_str(self) -> &'static str {
        use TerminalConfigurationConfigurationObject::*;
        match self {
            TerminalConfiguration => "terminal.configuration",
        }
    }
}

impl std::str::FromStr for TerminalConfigurationConfigurationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalConfigurationConfigurationObject::*;
        match s {
            "terminal.configuration" => Ok(TerminalConfiguration),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TerminalConfigurationConfigurationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalConfigurationConfigurationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalConfigurationConfigurationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalConfigurationConfigurationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalConfigurationConfigurationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TerminalConfigurationConfigurationObject"))
    }
}
impl stripe_types::Object for TerminalConfigurationConfiguration {
    type Id = stripe_terminal::terminal_configuration_configuration::TerminalConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TerminalConfigurationId, "tmc_");
pub mod requests;
