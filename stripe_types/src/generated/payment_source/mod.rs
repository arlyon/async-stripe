/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum PaymentSource {
    #[serde(rename = "account")]
    Account(stripe_types::Account),
    #[serde(rename = "bank_account")]
    BankAccount(stripe_types::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_types::Card),
    #[serde(rename = "source")]
    Source(stripe_types::Source),
}
impl stripe_types::Object for PaymentSource {
    type Id = String;
    fn id(&self) -> Self::Id {
        match self {
            Self::Account(v) => v.id.to_string(),
            Self::BankAccount(v) => v.id.to_string(),
            Self::Card(v) => v.id.to_string(),
            Self::Source(v) => v.id.to_string(),
        }
    }
}
