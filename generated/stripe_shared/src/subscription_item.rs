/// Subscription items allow you to create customer subscriptions with more than
/// one plan, making it easy to represent complex billing relationships.
///
/// For more details see <<https://stripe.com/docs/api/subscription_items/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionItemBillingThresholds>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionItemId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    pub plan: stripe_shared::Plan,
    pub price: stripe_shared::Price,
    /// The [quantity](https://stripe.com/docs/subscriptions/quantities) of the plan to which the customer should be subscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The `subscription` this `subscription_item` belongs to.
    pub subscription: String,
    /// The tax rates which apply to this `subscription_item`.
    /// When set, the `default_tax_rates` on the subscription do not apply to this `subscription_item`.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
}
impl stripe_types::Object for SubscriptionItem {
    type Id = stripe_shared::SubscriptionItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SubscriptionItemId);
