#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedLocation {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal::location::TerminalLocationId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedLocationObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedLocationObject {
    TerminalLocation,
}

impl DeletedLocationObject {
    pub fn as_str(self) -> &'static str {
        use DeletedLocationObject::*;
        match self {
            TerminalLocation => "terminal.location",
        }
    }
}

impl std::str::FromStr for DeletedLocationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedLocationObject::*;
        match s {
            "terminal.location" => Ok(TerminalLocation),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedLocationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedLocationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedLocationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedLocationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedLocationObject"))
    }
}
impl stripe_types::Object for DeletedLocation {
    type Id = stripe_terminal::terminal::location::TerminalLocationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
