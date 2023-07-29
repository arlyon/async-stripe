#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AlternateStatementDescriptors {
    /// The Kana variation of the descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kana: Option<String>,
    /// The Kanji variation of the descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AlternateStatementDescriptors {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
