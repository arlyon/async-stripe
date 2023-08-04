/// [Tax codes](https://stripe.com/docs/tax/tax-categories) classify goods and services for tax purposes.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxCode {
    /// A detailed description of which types of products the tax code represents.
    pub description: String,
    /// Unique identifier for the object.
    pub id: stripe_types::tax_product_resource_tax_code::TaxCodeId,
    /// A short name for the tax code.
    pub name: String,
}
impl stripe_types::Object for TaxProductResourceTaxCode {
    type Id = stripe_types::tax_product_resource_tax_code::TaxCodeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxCodeId, "txcd_");
