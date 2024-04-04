#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedInvoice {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::InvoiceId,
}
impl stripe_types::Object for DeletedInvoice {
    type Id = stripe_shared::InvoiceId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
