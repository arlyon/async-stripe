/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum DeletedExternalAccount {
    DeletedBankAccount(stripe_types::bank_account::DeletedBankAccount),
    DeletedCard(stripe_types::card::DeletedCard),
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
