#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NetworkData {
    /// Identifier assigned to the acquirer by the card network.
    ///
    /// Sometimes this value is not provided by the network; in this case, the value will be `null`.
    pub acquiring_institution_id: Option<String>,
}
