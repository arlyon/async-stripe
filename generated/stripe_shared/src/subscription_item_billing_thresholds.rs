#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionItemBillingThresholds {
    /// Usage threshold that triggers the subscription to create an invoice
    pub usage_gte: Option<i64>,
}
