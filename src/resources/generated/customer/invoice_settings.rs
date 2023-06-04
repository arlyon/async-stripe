#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceSettings {
    /// Default custom fields to be displayed on invoices for this customer.
    pub custom_fields: Option<Vec<crate::invoice::custom_field::CustomField>>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    pub default_payment_method: Option<crate::Expandable<crate::payment_method::PaymentMethod>>,
    /// Default footer to be displayed on invoices for this customer.
    pub footer: Option<String>,
    /// Default options for invoice PDF rendering for this customer.
    pub rendering_options: Option<crate::invoice::rendering_options::RenderingOptions>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceSettings {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
