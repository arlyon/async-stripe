/// A Configurations object represents how features should be configured for terminal readers.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Configuration {
#[serde(skip_serializing_if = "Option::is_none")]
pub bbpos_wisepos_e: Option<stripe_terminal::terminal::configuration::device_type_specific_config::DeviceTypeSpecificConfig>,
    /// Unique identifier for the object.
pub id: stripe_terminal::terminal::configuration::TerminalConfigurationId,
    /// Whether this Configuration is the default for your account.
pub is_account_default: Option<bool>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: ConfigurationObject,
#[serde(skip_serializing_if = "Option::is_none")]
pub tipping: Option<stripe_terminal::terminal::configuration::tipping::Tipping>,
#[serde(skip_serializing_if = "Option::is_none")]
pub verifone_p400: Option<stripe_terminal::terminal::configuration::device_type_specific_config::DeviceTypeSpecificConfig>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Configuration {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConfigurationObject {
    TerminalConfiguration,
}

impl ConfigurationObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TerminalConfiguration => "terminal.configuration",
        }
    }
}

impl std::str::FromStr for ConfigurationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "terminal.configuration" => Ok(Self::TerminalConfiguration),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ConfigurationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfigurationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ConfigurationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConfigurationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ConfigurationObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ConfigurationObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ConfigurationObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ConfigurationObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Configuration {
    type Id = stripe_terminal::terminal::configuration::TerminalConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TerminalConfigurationId, "tmc_");
pub mod deleted;
pub use deleted::DeletedConfiguration;
pub mod currency_specific_config;
pub use currency_specific_config::CurrencySpecificConfig;
pub mod device_type_specific_config;
pub use device_type_specific_config::DeviceTypeSpecificConfig;
pub mod tipping;
pub use tipping::Tipping;
