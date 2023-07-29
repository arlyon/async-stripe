/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum DeletedExternalAccount {
    DeletedBankAccount(stripe_types::bank_account::DeletedBankAccount),
    DeletedCard(stripe_types::card::DeletedCard),
}
#[cfg(feature = "min-ser")]
impl stripe_types::StripeDeserialize for DeletedExternalAccount {
    fn deserialize(str: &str) -> Result<Self, String> {
        use miniserde::json::from_str;
        use stripe_types::StripeDeserialize;
        let obj_name: stripe_types::deser::ObjectName =
            from_str(str).map_err(|_| "Missing `object` field")?;
        Ok(match obj_name.object.as_str() {
            "bank_account" => Self::DeletedBankAccount(StripeDeserialize::deserialize(str)?),
            "card" => Self::DeletedCard(StripeDeserialize::deserialize(str)?),
            _ => return Err("Unexpected object name".into()),
        })
    }
}

impl stripe_types::Object for DeletedExternalAccount {
    type Id = String;
    fn id(&self) -> Self::Id {
        match self {
            Self::DeletedBankAccount(v) => v.id.to_string(),
            Self::DeletedCard(v) => v.id.to_string(),
        }
    }
}
