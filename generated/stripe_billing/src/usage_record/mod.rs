/// Usage records allow you to report customer usage and metrics to Stripe for
/// metered billing of subscription prices.
///
/// Related guide: [Metered billing](https://stripe.com/docs/billing/subscriptions/metered-billing).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: stripe_billing::usage_record::UsageRecordId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The usage quantity for the specified date.
    pub quantity: u64,
    /// The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,
    /// The timestamp when this usage occurred.
    pub timestamp: stripe_types::Timestamp,
}
impl stripe_types::Object for UsageRecord {
    type Id = stripe_billing::usage_record::UsageRecordId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(UsageRecordId, "mbur_");
#[cfg(feature = "usage_record")]
mod requests;
#[cfg(feature = "usage_record")]
pub use requests::*;
