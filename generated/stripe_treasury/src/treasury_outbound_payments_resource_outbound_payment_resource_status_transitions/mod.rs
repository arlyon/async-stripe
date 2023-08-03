#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
    /// Timestamp describing when an OutboundPayment changed status to `canceled`.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `failed`.
    pub failed_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `posted`.
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `returned`.
    pub returned_at: Option<stripe_types::Timestamp>,
}
