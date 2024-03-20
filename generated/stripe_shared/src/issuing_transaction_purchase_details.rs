#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingTransactionPurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    pub flight: Option<stripe_shared::IssuingTransactionFlightData>,
    /// Information about fuel that was purchased with this transaction.
    pub fuel: Option<stripe_shared::IssuingTransactionFuelData>,
    /// Information about lodging that was purchased with this transaction.
    pub lodging: Option<stripe_shared::IssuingTransactionLodgingData>,
    /// The line items in the purchase.
    pub receipt: Option<Vec<stripe_shared::IssuingTransactionReceiptData>>,
    /// A merchant-specific order number.
    pub reference: Option<String>,
}
