#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalInvoiceList {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
