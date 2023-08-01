#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Installments {
    /// Whether Installments are enabled for this Invoice.
    pub enabled: Option<bool>,
}
