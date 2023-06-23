#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BillingThresholds {
    /// Usage threshold that triggers the subscription to create an invoice.
    pub usage_gte: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BillingThresholds {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
