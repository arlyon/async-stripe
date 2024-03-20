#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedSubscriptionItem {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionItemId,
}
impl stripe_types::Object for DeletedSubscriptionItem {
    type Id = stripe_shared::SubscriptionItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
