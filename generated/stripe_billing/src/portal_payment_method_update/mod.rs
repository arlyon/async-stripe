#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
