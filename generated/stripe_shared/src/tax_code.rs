/// [Tax codes](https://stripe.com/docs/tax/tax-categories) classify goods and services for tax purposes.
///
/// For more details see <<https://stripe.com/docs/api/tax_codes/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxCode {
    /// A detailed description of which types of products the tax code represents.
    pub description: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::TaxCodeId,
    /// A short name for the tax code.
    pub name: String,
}
impl stripe_types::Object for TaxCode {
    type Id = stripe_shared::TaxCodeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxCodeId);
