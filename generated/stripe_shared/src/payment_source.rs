/// The resource representing a Stripe Polymorphic
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum PaymentSource {
    #[serde(rename = "account")]
    Account(stripe_shared::Account),
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[serde(rename = "source")]
    Source(stripe_shared::Source),
}
impl stripe_types::Object for PaymentSource {
    type Id = smol_str::SmolStr;
    fn id(&self) -> &Self::Id {
        match self {
            Self::Account(v) => v.id.inner(),
            Self::BankAccount(v) => v.id.inner(),
            Self::Card(v) => v.id.inner(),
            Self::Source(v) => v.id.inner(),
        }
    }
}
