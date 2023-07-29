#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AlternateStatementDescriptors {
    /// The Kana variation of the descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kana: Option<String>,
    /// The Kanji variation of the descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<String>,
}
