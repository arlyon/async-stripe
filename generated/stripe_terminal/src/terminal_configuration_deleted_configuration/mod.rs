#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalConfigurationDeletedConfiguration {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_configuration_configuration::TerminalConfigurationId,
}
impl stripe_types::Object for TerminalConfigurationDeletedConfiguration {
    type Id = stripe_terminal::terminal_configuration_configuration::TerminalConfigurationId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
