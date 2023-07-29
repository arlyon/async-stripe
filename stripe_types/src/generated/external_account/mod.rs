/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum ExternalAccount {
    BankAccount(stripe_types::bank_account::BankAccount),
    Card(stripe_types::card::Card),
}
#[cfg(feature = "min-ser")]
impl stripe_types::StripeDeserialize for ExternalAccount {
    fn deserialize(str: &str) -> Result<Self, String> {
        use miniserde::json::from_str;
        use stripe_types::StripeDeserialize;
        let obj_name: stripe_types::deser::ObjectName =
            from_str(str).map_err(|_| "Missing `object` field")?;
        Ok(match obj_name.object.as_str() {
            "bank_account" => Self::BankAccount(StripeDeserialize::deserialize(str)?),
            "card" => Self::Card(StripeDeserialize::deserialize(str)?),
            _ => return Err("Unexpected object name".into()),
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
pub use deleted::DeletedExternalAccount;
