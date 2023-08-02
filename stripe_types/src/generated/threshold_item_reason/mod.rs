#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ThresholdItemReason {
    /// The IDs of the line items that triggered the threshold invoice.
    pub line_item_ids: Vec<String>,
    /// The quantity threshold boundary that applied to the given line item.
    pub usage_gte: i64,
}
