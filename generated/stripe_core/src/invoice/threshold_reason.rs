#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ThresholdReason {
    /// The total invoice amount threshold boundary if it triggered the threshold invoice.
    pub amount_gte: Option<i64>,
    /// Indicates which line items triggered a threshold invoice.
    pub item_reasons: Vec<stripe_core::invoice::threshold_item_reason::ThresholdItemReason>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThresholdReason {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
