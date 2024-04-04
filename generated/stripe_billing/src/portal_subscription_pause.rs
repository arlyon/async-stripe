#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalSubscriptionPause {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
