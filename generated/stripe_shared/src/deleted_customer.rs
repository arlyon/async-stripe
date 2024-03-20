#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedCustomer {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::CustomerId,
}
impl stripe_types::Object for DeletedCustomer {
    type Id = stripe_shared::CustomerId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
