#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceTypeEps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
