/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum ExternalAccount {
    BankAccount(stripe_core::bank_account::BankAccount),
    Card(stripe_core::card::Card),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for ExternalAccount {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;
        let obj_name: crate::deser::ObjectName = from_str(str)?;
        Ok(match obj_name.object.as_str() {
            "bank_account" => Self::BankAccount(from_str(str)?),
            "card" => Self::Card(from_str(str)?),
            _ => return Err(crate::StripeError::JSONDeserialize("Unexpected object name".into())),
        })
    }
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
pub mod requests;
pub use deleted::DeletedExternalAccount;
