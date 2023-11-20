#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceTransferData {
    /// The amount in cents (or local equivalent) that will be transferred to the destination account.
    ///
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    /// The connected account receiving the transfer.
    pub destination: stripe_types::Expandable<stripe_types::Account>,
}
