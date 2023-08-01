#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedConfiguration {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal::configuration::TerminalConfigurationId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedConfigurationObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedConfigurationObject {
    TerminalConfiguration,
}

impl DeletedConfigurationObject {
    pub fn as_str(self) -> &'static str {
        use DeletedConfigurationObject::*;
        match self {
            TerminalConfiguration => "terminal.configuration",
        }
    }
}

impl std::str::FromStr for DeletedConfigurationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedConfigurationObject::*;
        match s {
            "terminal.configuration" => Ok(TerminalConfiguration),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedConfigurationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedConfigurationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedConfigurationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedConfigurationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedConfigurationObject"))
    }
}
impl stripe_types::Object for DeletedConfiguration {
    type Id = stripe_terminal::terminal::configuration::TerminalConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
