#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TransferData {
    /// The amount that will be transferred automatically when the order is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    pub amount: Option<i64>,
    /// ID of the Connected account receiving the transfer.
    pub destination: stripe_types::Expandable<stripe_types::account::Account>,
}
