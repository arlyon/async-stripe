#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlatformTaxFee {
    /// The Connected account that incurred this charge.
    pub account: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::PlatformTaxFeeId,
    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,
    /// The type of tax (VAT).
    #[serde(rename = "type")]
    pub type_: String,
}
impl stripe_types::Object for PlatformTaxFee {
    type Id = stripe_shared::PlatformTaxFeeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PlatformTaxFeeId, "ptf_");
