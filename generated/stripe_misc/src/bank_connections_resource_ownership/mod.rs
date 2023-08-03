/// Describes a snapshot of the owners of an account at a particular point in time.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceOwnership {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::bank_connections_resource_ownership::FinancialConnectionsAccountOwnershipId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: BankConnectionsResourceOwnershipObject,
    /// A paginated list of owners for this account.
    pub owners: stripe_types::List<stripe_misc::BankConnectionsResourceOwner>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceOwnershipObject {
    FinancialConnectionsAccountOwnership,
}

impl BankConnectionsResourceOwnershipObject {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceOwnershipObject::*;
        match self {
            FinancialConnectionsAccountOwnership => "financial_connections.account_ownership",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceOwnershipObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceOwnershipObject::*;
        match s {
            "financial_connections.account_ownership" => Ok(FinancialConnectionsAccountOwnership),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankConnectionsResourceOwnershipObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceOwnershipObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceOwnershipObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceOwnershipObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceOwnershipObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceOwnershipObject"))
    }
}
impl stripe_types::Object for BankConnectionsResourceOwnership {
    type Id = stripe_misc::bank_connections_resource_ownership::FinancialConnectionsAccountOwnershipId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FinancialConnectionsAccountOwnershipId);
