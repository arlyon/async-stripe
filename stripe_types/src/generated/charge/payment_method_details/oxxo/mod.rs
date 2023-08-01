#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Oxxo {
    /// OXXO reference number.
    pub number: Option<String>,
}
