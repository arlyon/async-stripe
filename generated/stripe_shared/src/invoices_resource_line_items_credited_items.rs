#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoicesResourceLineItemsCreditedItems {
    /// Invoice containing the credited invoice line items
    pub invoice: String,
    /// Credited invoice line items
    pub invoice_line_items: Vec<String>,
}
