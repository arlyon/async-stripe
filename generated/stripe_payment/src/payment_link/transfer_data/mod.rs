#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TransferData {
    /// The amount in %s that will be transferred to the destination account.
    ///
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    /// The connected account receiving the transfer.
    pub destination: stripe_types::Expandable<stripe_types::account::Account>,
}
