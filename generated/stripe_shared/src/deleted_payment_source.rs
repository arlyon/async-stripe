/// The resource representing a Stripe Polymorphic
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum DeletedPaymentSource {
    #[serde(rename = "bank_account")]
    DeletedBankAccount(stripe_shared::DeletedBankAccount),
    #[serde(rename = "card")]
    DeletedCard(stripe_shared::DeletedCard),
}
impl stripe_types::Object for DeletedPaymentSource {
    type Id = smol_str::SmolStr;
    fn id(&self) -> &Self::Id {
        match self {
            Self::DeletedBankAccount(v) => v.id.inner(),
            Self::DeletedCard(v) => v.id.inner(),
        }
    }
}
