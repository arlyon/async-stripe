#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectEmbeddedPaymentsFeatures {
    /// Whether to allow capturing and cancelling payment intents. This is `true` by default.
    pub capture_payments: bool,
    /// Whether to allow responding to disputes, including submitting evidence and accepting disputes.
    /// This is `true` by default.
    pub dispute_management: bool,
    /// Whether to allow sending refunds. This is `true` by default.
    pub refund_management: bool,
}
