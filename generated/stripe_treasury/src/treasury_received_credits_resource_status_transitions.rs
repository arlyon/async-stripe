#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedCreditsResourceStatusTransitions {
    /// Timestamp describing when the CreditReversal changed status to `posted`
    pub posted_at: Option<stripe_types::Timestamp>,
}
