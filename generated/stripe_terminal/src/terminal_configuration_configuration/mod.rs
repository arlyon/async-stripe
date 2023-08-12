/// A Configurations object represents how features should be configured for terminal readers.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalConfigurationConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_configuration_configuration::TerminalConfigurationId,
    /// Whether this Configuration is the default for your account.
    pub is_account_default: Option<bool>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
}
impl stripe_types::Object for TerminalConfigurationConfiguration {
    type Id = stripe_terminal::terminal_configuration_configuration::TerminalConfigurationId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TerminalConfigurationId, "tmc_");
#[cfg(feature = "terminal_configuration_configuration")]
mod requests;
#[cfg(feature = "terminal_configuration_configuration")]
pub use requests::*;
