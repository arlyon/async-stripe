#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsResourceBillingCycleAnchorConfig {
    /// The day of the month of the billing_cycle_anchor.
    pub day_of_month: i64,
    /// The hour of the day of the billing_cycle_anchor.
    pub hour: Option<i64>,
    /// The minute of the hour of the billing_cycle_anchor.
    pub minute: Option<i64>,
    /// The month to start full cycle billing periods.
    pub month: Option<i64>,
    /// The second of the minute of the billing_cycle_anchor.
    pub second: Option<i64>,
}
