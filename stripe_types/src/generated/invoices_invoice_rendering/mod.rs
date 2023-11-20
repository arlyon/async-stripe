#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoicesInvoiceRendering {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
    /// Invoice pdf rendering options.
    pub pdf: Option<stripe_types::InvoiceRenderingPdf>,
}
