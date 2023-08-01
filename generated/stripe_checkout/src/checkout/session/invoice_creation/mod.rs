#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceCreation {
    /// Indicates whether invoice creation is enabled for the Checkout Session.
    pub enabled: bool,
    pub invoice_data:
        stripe_checkout::checkout::session::invoice_creation::invoice_data::InvoiceData,
}
pub mod invoice_data;
pub use invoice_data::InvoiceData;
