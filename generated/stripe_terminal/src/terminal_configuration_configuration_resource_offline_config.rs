#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TerminalConfigurationConfigurationResourceOfflineConfig {
    /// Determines whether to allow transactions to be collected while reader is offline.
    /// Defaults to false.
    pub enabled: Option<bool>,
}
