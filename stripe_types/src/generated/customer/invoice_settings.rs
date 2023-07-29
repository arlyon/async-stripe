#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoiceSettings {
    /// Default custom fields to be displayed on invoices for this customer.
    pub custom_fields: Option<Vec<stripe_types::invoice::custom_field::CustomField>>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    pub default_payment_method:
        Option<stripe_types::Expandable<stripe_types::payment_method::PaymentMethod>>,
    /// Default footer to be displayed on invoices for this customer.
    pub footer: Option<String>,
    /// Default options for invoice PDF rendering for this customer.
    pub rendering_options: Option<stripe_types::invoice::rendering_options::RenderingOptions>,
}
