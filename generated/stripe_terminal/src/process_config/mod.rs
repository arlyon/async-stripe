/// Represents a per-transaction override of a reader configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ProcessConfig {
    /// Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<stripe_terminal::tipping_config::TippingConfig>,
}
