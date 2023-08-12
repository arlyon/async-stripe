#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalLocationDeletedLocation {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_location_location::TerminalLocationId,
}
impl stripe_types::Object for TerminalLocationDeletedLocation {
    type Id = stripe_terminal::terminal_location_location::TerminalLocationId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
