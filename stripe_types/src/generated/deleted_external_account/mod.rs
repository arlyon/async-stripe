/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum DeletedExternalAccount {
    #[serde(rename = "bank_account")]
    DeletedBankAccount(stripe_types::DeletedBankAccount),
    #[serde(rename = "card")]
    DeletedCard(stripe_types::DeletedCard),
}
impl stripe_types::Object for DeletedExternalAccount {
    type Id = String;
    fn id(&self) -> Self::Id {
        match self {
            Self::DeletedBankAccount(v) => v.id.to_string(),
            Self::DeletedCard(v) => v.id.to_string(),
        }
    }
}
