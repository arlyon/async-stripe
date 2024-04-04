#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedDebitsResourceStatusTransitions {
    /// Timestamp describing when the DebitReversal changed status to `completed`.
    pub completed_at: Option<stripe_types::Timestamp>,
}
