#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedAccount {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::AccountId,
}
impl stripe_types::Object for DeletedAccount {
    type Id = stripe_shared::AccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
