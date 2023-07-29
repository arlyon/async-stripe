/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ExternalAccount {
    BankAccount(stripe_types::bank_account::BankAccount),
    Card(stripe_types::card::Card),
}
impl stripe_types::Object for ExternalAccount {
    type Id = String;
    fn id(&self) -> Self::Id {
        match self {
            Self::BankAccount(v) => v.id.to_string(),
            Self::Card(v) => v.id.to_string(),
        }
    }
}
pub mod deleted;
pub use deleted::DeletedExternalAccount;
