/// A Location represents a grouping of readers.
///
/// Related guide: [Fleet management](https://stripe.com/docs/terminal/fleet/locations).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalLocationLocation {
    pub address: stripe_types::Address,
    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,
    /// The display name of the location.
    pub display_name: String,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_location_location::TerminalLocationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TerminalLocationLocationObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalLocationLocationObject {
    TerminalLocation,
}

impl TerminalLocationLocationObject {
    pub fn as_str(self) -> &'static str {
        use TerminalLocationLocationObject::*;
        match self {
            TerminalLocation => "terminal.location",
        }
    }
}

impl std::str::FromStr for TerminalLocationLocationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalLocationLocationObject::*;
        match s {
            "terminal.location" => Ok(TerminalLocation),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TerminalLocationLocationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalLocationLocationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalLocationLocationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalLocationLocationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalLocationLocationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TerminalLocationLocationObject"))
    }
}
impl stripe_types::Object for TerminalLocationLocation {
    type Id = stripe_terminal::terminal_location_location::TerminalLocationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TerminalLocationId, "tml_");
pub mod requests;
