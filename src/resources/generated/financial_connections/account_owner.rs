#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountOwner {
    /// The email address of the owner.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: crate::financial_connections::account_owner::FinancialConnectionsAccountOwnerId,
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
    pub refreshed_at: Option<crate::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountOwner {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AccountOwnerObject {
    #[serde(rename = "financial_connections.account_owner")]
    FinancialConnectionsAccountOwner,
}

impl AccountOwnerObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialConnectionsAccountOwner => "financial_connections.account_owner",
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
impl crate::Object for AccountOwner {
    type Id = crate::financial_connections::account_owner::FinancialConnectionsAccountOwnerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(FinancialConnectionsAccountOwnerId);
