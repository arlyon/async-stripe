/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PaymentSource {
    Account(stripe_types::account::Account),
    BankAccount(stripe_types::bank_account::BankAccount),
    Card(stripe_types::card::Card),
    Source(stripe_types::source::Source),
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
pub mod deleted;
pub use deleted::DeletedPaymentSource;
