#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoiceSettingCustomerSetting {
    /// Default custom fields to be displayed on invoices for this customer.
    pub custom_fields: Option<Vec<stripe_shared::InvoiceSettingCustomField>>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    pub default_payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// Default footer to be displayed on invoices for this customer.
    pub footer: Option<String>,
    /// Default options for invoice PDF rendering for this customer.
    pub rendering_options: Option<stripe_shared::InvoiceSettingRenderingOptions>,
}
