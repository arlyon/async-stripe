#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedSubscriptionItem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::subscription_item::SubscriptionItemId,
}
impl stripe_types::Object for DeletedSubscriptionItem {
    type Id = stripe_types::subscription_item::SubscriptionItemId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
