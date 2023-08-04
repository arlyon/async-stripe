#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedProduct {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::product::ProductId,
}
impl stripe_types::Object for DeletedProduct {
    type Id = stripe_types::product::ProductId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
