#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionPause {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
