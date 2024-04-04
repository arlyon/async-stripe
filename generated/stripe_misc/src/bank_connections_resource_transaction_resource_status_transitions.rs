#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceTransactionResourceStatusTransitions {
    /// Time at which this transaction posted. Measured in seconds since the Unix epoch.
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Time at which this transaction was voided. Measured in seconds since the Unix epoch.
    pub void_at: Option<stripe_types::Timestamp>,
}
