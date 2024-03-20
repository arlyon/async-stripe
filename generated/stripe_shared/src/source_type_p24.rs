#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceTypeP24 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
