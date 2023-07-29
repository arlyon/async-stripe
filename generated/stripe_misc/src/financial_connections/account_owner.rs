#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountOwner {
    /// The email address of the owner.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::financial_connections::account_owner::FinancialConnectionsAccountOwnerId,
    /// The full name of the owner.
    pub name: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: AccountOwnerObject,
    /// The ownership object that this owner belongs to.
    pub ownership: String,
    /// The raw phone number of the owner.
    pub phone: Option<String>,
    /// The raw physical address of the owner.
    pub raw_address: Option<String>,
    /// The timestamp of the refresh that updated this owner.
    pub refreshed_at: Option<stripe_types::Timestamp>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountOwnerObject {
    FinancialConnectionsAccountOwner,
}

impl AccountOwnerObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialConnectionsAccountOwner => "financial_connections.account_owner",
        }
    }
}

impl std::str::FromStr for AccountOwnerObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "financial_connections.account_owner" => Ok(Self::FinancialConnectionsAccountOwner),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountOwnerObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountOwnerObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AccountOwnerObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountOwnerObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountOwnerObject"))
    }
}
impl stripe_types::Object for AccountOwner {
    type Id = stripe_misc::financial_connections::account_owner::FinancialConnectionsAccountOwnerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FinancialConnectionsAccountOwnerId);
