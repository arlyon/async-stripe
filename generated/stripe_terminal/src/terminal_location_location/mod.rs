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
}
impl stripe_types::Object for TerminalLocationLocation {
    type Id = stripe_terminal::terminal_location_location::TerminalLocationId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TerminalLocationId, "tml_");
#[cfg(feature = "terminal_location_location")]
mod requests;
#[cfg(feature = "terminal_location_location")]
pub use requests::*;
