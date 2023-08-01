#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedBankAccount {
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::bank_account::BankAccountId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedBankAccountObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedBankAccountObject {
    BankAccount,
}

impl DeletedBankAccountObject {
    pub fn as_str(self) -> &'static str {
        use DeletedBankAccountObject::*;
        match self {
            BankAccount => "bank_account",
        }
    }
}

impl std::str::FromStr for DeletedBankAccountObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedBankAccountObject::*;
        match s {
            "bank_account" => Ok(BankAccount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedBankAccountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedBankAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedBankAccountObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedBankAccountObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedBankAccountObject"))
    }
}
impl stripe_types::Object for DeletedBankAccount {
    type Id = stripe_types::bank_account::BankAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
