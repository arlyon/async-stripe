#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceTransactionPaperCheckData {
    /// Time at which the deposited funds will be available for use.
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_at: Option<String>,
    /// Comma-separated list of invoice IDs associated with the paper check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<String>,
}
