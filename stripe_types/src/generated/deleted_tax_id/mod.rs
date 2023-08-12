#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedTaxId {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::tax_id::TaxIdId,
}
impl stripe_types::Object for DeletedTaxId {
    type Id = stripe_types::tax_id::TaxIdId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
