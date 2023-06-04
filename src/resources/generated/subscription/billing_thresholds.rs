#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BillingThresholds {
    /// Monetary threshold that triggers the subscription to create an invoice.
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    ///
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    /// This value may not be `true` if the subscription contains items with plans that have `aggregate_usage=last_ever`.
    pub reset_billing_cycle_anchor: Option<bool>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BillingThresholds {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
