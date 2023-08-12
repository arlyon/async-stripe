#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedPerson {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::person::PersonId,
}
impl stripe_types::Object for DeletedPerson {
    type Id = stripe_types::person::PersonId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
