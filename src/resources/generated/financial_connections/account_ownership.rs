/// Describes a snapshot of the owners of an account at a particular point in time.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountOwnership {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Unique identifier for the object.
    pub id: crate::financial_connections::account_ownership::FinancialConnectionsAccountOwnershipId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: AccountOwnershipObject,
    /// A paginated list of owners for this account.
    pub owners: crate::List<crate::financial_connections::account_owner::AccountOwner>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountOwnership {
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
pub enum AccountOwnershipObject {
    #[serde(rename = "financial_connections.account_ownership")]
    FinancialConnectionsAccountOwnership,
}

impl AccountOwnershipObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialConnectionsAccountOwnership => "financial_connections.account_ownership",
        }
    }
}

impl AsRef<str> for AccountOwnershipObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountOwnershipObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for AccountOwnership {
    type Id =
        crate::financial_connections::account_ownership::FinancialConnectionsAccountOwnershipId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(FinancialConnectionsAccountOwnershipId);
