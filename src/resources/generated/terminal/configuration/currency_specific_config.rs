#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CurrencySpecificConfig {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,
    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CurrencySpecificConfig {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
