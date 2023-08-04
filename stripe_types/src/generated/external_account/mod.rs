/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum ExternalAccount {
    #[serde(rename = "bank_account")]
    BankAccount(stripe_types::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_types::Card),
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
