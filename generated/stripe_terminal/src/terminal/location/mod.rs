/// A Location represents a grouping of readers.
///
/// Related guide: [Fleet Management](https://stripe.com/docs/terminal/fleet/locations).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Location {
    pub address: stripe_types::address::Address,
    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,
    /// The display name of the location.
    pub display_name: String,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal::location::TerminalLocationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: LocationObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Location {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum LocationObject {
    #[serde(rename = "terminal.location")]
    TerminalLocation,
}

impl LocationObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TerminalLocation => "terminal.location",
        }
    }
}

impl AsRef<str> for LocationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LocationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Location {
    type Id = stripe_terminal::terminal::location::TerminalLocationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TerminalLocationId, "tml_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedLocation;
