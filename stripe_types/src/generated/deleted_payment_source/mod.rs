/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum DeletedPaymentSource {
    #[serde(rename = "bank_account")]
    DeletedBankAccount(stripe_types::DeletedBankAccount),
    #[serde(rename = "card")]
    DeletedCard(stripe_types::DeletedCard),
}
impl stripe_types::Object for DeletedPaymentSource {
    type Id = String;
    fn id(&self) -> Option<&str> {
        match self {
            Self::DeletedBankAccount(v) => Some(v.id.as_str()),
            Self::DeletedCard(v) => Some(v.id.as_str()),
        }
    }
}
