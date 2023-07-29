#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
