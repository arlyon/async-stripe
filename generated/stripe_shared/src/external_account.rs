/// The resource representing a Stripe Polymorphic
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum ExternalAccount {
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
}
impl stripe_types::Object for ExternalAccount {
    type Id = smol_str::SmolStr;
    fn id(&self) -> &Self::Id {
        match self {
            Self::BankAccount(v) => v.id.inner(),
            Self::Card(v) => v.id.inner(),
        }
    }
}
