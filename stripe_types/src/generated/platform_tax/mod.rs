#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlatformTax {
    /// The Connected account that incurred this charge.
    pub account: String,
    /// Unique identifier for the object.
    pub id: stripe_types::platform_tax::PlatformTaxFeeId,
    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,
    /// The type of tax (VAT).
    #[serde(rename = "type")]
    pub type_: String,
}
impl stripe_types::Object for PlatformTax {
    type Id = stripe_types::platform_tax::PlatformTaxFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PlatformTaxFeeId, "ptf_");
