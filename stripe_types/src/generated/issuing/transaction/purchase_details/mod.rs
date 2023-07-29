#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    pub flight: Option<stripe_types::issuing::transaction::purchase_details::flight::Flight>,
    /// Information about fuel that was purchased with this transaction.
    pub fuel: Option<stripe_types::issuing::transaction::purchase_details::fuel::Fuel>,
    /// Information about lodging that was purchased with this transaction.
    pub lodging: Option<stripe_types::issuing::transaction::purchase_details::lodging::Lodging>,
    /// The line items in the purchase.
    pub receipt:
        Option<Vec<stripe_types::issuing::transaction::purchase_details::receipt::Receipt>>,
    /// A merchant-specific order number.
    pub reference: Option<String>,
}
pub mod flight;
pub use flight::Flight;
pub mod fuel;
pub use fuel::Fuel;
pub mod lodging;
pub use lodging::Lodging;
pub mod receipt;
pub use receipt::Receipt;
