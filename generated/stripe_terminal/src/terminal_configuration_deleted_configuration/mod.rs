#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalConfigurationDeletedConfiguration {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_configuration_configuration::TerminalConfigurationId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TerminalConfigurationDeletedConfigurationObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalConfigurationDeletedConfigurationObject {
    TerminalConfiguration,
}

impl TerminalConfigurationDeletedConfigurationObject {
    pub fn as_str(self) -> &'static str {
        use TerminalConfigurationDeletedConfigurationObject::*;
        match self {
            TerminalConfiguration => "terminal.configuration",
        }
    }
}

impl std::str::FromStr for TerminalConfigurationDeletedConfigurationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalConfigurationDeletedConfigurationObject::*;
        match s {
            "terminal.configuration" => Ok(TerminalConfiguration),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TerminalConfigurationDeletedConfigurationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalConfigurationDeletedConfigurationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalConfigurationDeletedConfigurationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalConfigurationDeletedConfigurationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalConfigurationDeletedConfigurationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TerminalConfigurationDeletedConfigurationObject"))
    }
}
impl stripe_types::Object for TerminalConfigurationDeletedConfiguration {
    type Id = stripe_terminal::terminal_configuration_configuration::TerminalConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
