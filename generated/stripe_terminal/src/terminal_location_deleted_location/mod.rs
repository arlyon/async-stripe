#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalLocationDeletedLocation {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_location_location::TerminalLocationId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TerminalLocationDeletedLocationObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalLocationDeletedLocationObject {
    TerminalLocation,
}

impl TerminalLocationDeletedLocationObject {
    pub fn as_str(self) -> &'static str {
        use TerminalLocationDeletedLocationObject::*;
        match self {
            TerminalLocation => "terminal.location",
        }
    }
}

impl std::str::FromStr for TerminalLocationDeletedLocationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalLocationDeletedLocationObject::*;
        match s {
            "terminal.location" => Ok(TerminalLocation),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TerminalLocationDeletedLocationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalLocationDeletedLocationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalLocationDeletedLocationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalLocationDeletedLocationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalLocationDeletedLocationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TerminalLocationDeletedLocationObject"))
    }
}
impl stripe_types::Object for TerminalLocationDeletedLocation {
    type Id = stripe_terminal::terminal_location_location::TerminalLocationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
