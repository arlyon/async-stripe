/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum PaymentSource {
    Account(stripe_types::account::Account),
    BankAccount(stripe_types::bank_account::BankAccount),
    Card(stripe_types::card::Card),
    Source(stripe_types::source::Source),
}
#[cfg(feature = "min-ser")]
impl stripe_types::StripeDeserialize for PaymentSource {
    fn deserialize(str: &str) -> Result<Self, String> {
        use miniserde::json::from_str;
        use stripe_types::StripeDeserialize;
        let obj_name: stripe_types::deser::ObjectName =
            from_str(str).map_err(|_| "Missing `object` field")?;
        Ok(match obj_name.object.as_str() {
            "account" => Self::Account(StripeDeserialize::deserialize(str)?),
            "bank_account" => Self::BankAccount(StripeDeserialize::deserialize(str)?),
            "card" => Self::Card(StripeDeserialize::deserialize(str)?),
            "source" => Self::Source(StripeDeserialize::deserialize(str)?),
            _ => return Err("Unexpected object name".into()),
        })
    }
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
