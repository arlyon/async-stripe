#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceInvoiceSettings {
    /// The account tax IDs associated with the invoice.
    pub account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_types::TaxId>>>,
    /// A list of up to 4 custom fields to be displayed on the invoice.
    pub custom_fields: Option<Vec<stripe_types::InvoiceSettingCustomField>>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// Footer to be displayed on the invoice.
    pub footer: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Options for invoice PDF rendering.
    pub rendering_options: Option<stripe_types::InvoiceSettingRenderingOptions>,
}
