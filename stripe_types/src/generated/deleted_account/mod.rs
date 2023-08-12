#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedAccount {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::account::AccountId,
}
impl stripe_types::Object for DeletedAccount {
    type Id = stripe_types::account::AccountId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
