/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum DeletedPaymentSource {
    DeletedBankAccount(crate::bank_account::DeletedBankAccount),
    DeletedCard(crate::card::DeletedCard),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for DeletedPaymentSource {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;
        let obj_name: crate::deser::ObjectName = from_str(str)?;
        Ok(match obj_name.object.as_str() {
            "bank_account" => Self::DeletedBankAccount(from_str(str)?),
            "card" => Self::DeletedCard(from_str(str)?),
            _ => return Err(crate::StripeError::JSONDeserialize("Unexpected object name".into())),
        })
    }
}

impl crate::Object for DeletedPaymentSource {
    type Id = String;
    fn id(&self) -> Self::Id {
        match self {
            Self::DeletedBankAccount(v) => v.id.to_string(),
            Self::DeletedCard(v) => v.id.to_string(),
        }
    }
}
