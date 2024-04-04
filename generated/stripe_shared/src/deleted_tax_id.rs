#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedTaxId {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::TaxIdId,
}
impl stripe_types::Object for DeletedTaxId {
    type Id = stripe_shared::TaxIdId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
