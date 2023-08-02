#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    pub flight: Option<stripe_types::flight::Flight>,
    /// Information about fuel that was purchased with this transaction.
    pub fuel: Option<stripe_types::fuel::Fuel>,
    /// Information about lodging that was purchased with this transaction.
    pub lodging: Option<stripe_types::lodging::Lodging>,
    /// The line items in the purchase.
    pub receipt: Option<Vec<stripe_types::receipt::Receipt>>,
    /// A merchant-specific order number.
    pub reference: Option<String>,
}
