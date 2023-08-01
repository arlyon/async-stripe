#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceCreation {
    /// Enable creating an invoice on successful payment.
    pub enabled: bool,
    /// Configuration for the invoice.
    ///
    /// Default invoice values will be used if unspecified.
    pub invoice_data: Option<stripe_payment::payment_link::invoice_settings::InvoiceSettings>,
}
