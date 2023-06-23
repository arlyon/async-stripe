/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum PaymentSource {
    Account(stripe_core::account::Account),
    BankAccount(stripe_core::bank_account::BankAccount),
    Card(stripe_core::card::Card),
    Source(stripe_core::source::Source),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for PaymentSource {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;
        let obj_name: crate::deser::ObjectName = from_str(str)?;
        Ok(match obj_name.object.as_str() {
            "account" => Self::Account(from_str(str)?),
            "bank_account" => Self::BankAccount(from_str(str)?),
            "card" => Self::Card(from_str(str)?),
            "source" => Self::Source(from_str(str)?),
            _ => return Err(crate::StripeError::JSONDeserialize("Unexpected object name".into())),
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
pub mod requests;
pub use deleted::DeletedPaymentSource;
