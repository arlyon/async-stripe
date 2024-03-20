#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoiceSettingRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
}
