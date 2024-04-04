#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoiceInstallmentsCard {
    /// Whether Installments are enabled for this Invoice.
    pub enabled: Option<bool>,
}
