#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ThresholdItemReason {
    /// The IDs of the line items that triggered the threshold invoice.
    pub line_item_ids: Vec<String>,
    /// The quantity threshold boundary that applied to the given line item.
    pub usage_gte: i64,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThresholdItemReason {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
