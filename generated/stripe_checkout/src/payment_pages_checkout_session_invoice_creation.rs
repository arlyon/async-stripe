#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionInvoiceCreation {
    /// Indicates whether invoice creation is enabled for the Checkout Session.
    pub enabled: bool,
    pub invoice_data: stripe_checkout::PaymentPagesCheckoutSessionInvoiceSettings,
}
