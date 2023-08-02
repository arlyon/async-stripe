#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
