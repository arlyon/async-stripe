#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Tip {
    /// Portion of the amount that corresponds to a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Tip {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
