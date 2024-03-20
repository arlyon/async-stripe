#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectEmbeddedPaymentsConfig {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    pub features: stripe_connect::ConnectEmbeddedPaymentsFeatures,
}
