#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
    /// Timestamp describing when the Transaction changed status to `posted`.
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when the Transaction changed status to `void`.
    pub void_at: Option<stripe_types::Timestamp>,
}
