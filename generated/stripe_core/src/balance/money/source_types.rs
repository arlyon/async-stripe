#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypes {
    /// Amount for bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<i64>,
    /// Amount for card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<i64>,
    /// Amount for FPX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SourceTypes {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
