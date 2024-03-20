#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceLinkAccountSessionFilters {
    /// List of countries from which to filter accounts.
    pub countries: Option<Vec<String>>,
}
