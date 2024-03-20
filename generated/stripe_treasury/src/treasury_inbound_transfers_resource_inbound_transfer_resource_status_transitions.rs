#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    /// Timestamp describing when an InboundTransfer changed status to `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an InboundTransfer changed status to `failed`.
    pub failed_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an InboundTransfer changed status to `succeeded`.
    pub succeeded_at: Option<stripe_types::Timestamp>,
}
