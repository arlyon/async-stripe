#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingTransactionPurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    pub flight: Option<stripe_types::IssuingTransactionFlightData>,
    /// Information about fuel that was purchased with this transaction.
    pub fuel: Option<stripe_types::IssuingTransactionFuelData>,
    /// Information about lodging that was purchased with this transaction.
    pub lodging: Option<stripe_types::IssuingTransactionLodgingData>,
    /// The line items in the purchase.
    pub receipt: Option<Vec<stripe_types::IssuingTransactionReceiptData>>,
    /// A merchant-specific order number.
    pub reference: Option<String>,
}
