#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedInvoiceItem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::invoice_item::InvoiceitemId,
}
impl stripe_types::Object for DeletedInvoiceItem {
    type Id = stripe_types::invoice_item::InvoiceitemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
