/// Describes a snapshot of the owners of an account at a particular point in time.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountOwnership {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
pub id: stripe_misc::financial_connections::account_ownership::FinancialConnectionsAccountOwnershipId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: AccountOwnershipObject,
    /// A paginated list of owners for this account.
pub owners: stripe_types::List<stripe_misc::financial_connections::account_owner::AccountOwner>,

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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountOwnershipObject {
    FinancialConnectionsAccountOwnership,
}

impl AccountOwnershipObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialConnectionsAccountOwnership => "financial_connections.account_ownership",
        }
    }
}

impl std::str::FromStr for AccountOwnershipObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "financial_connections.account_ownership" => {
                Ok(Self::FinancialConnectionsAccountOwnership)
            }

            _ => Err(()),
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
impl serde::Serialize for AccountOwnershipObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountOwnershipObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountOwnershipObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountOwnershipObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<AccountOwnershipObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountOwnershipObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for AccountOwnership {
    type Id = stripe_misc::financial_connections::account_ownership::FinancialConnectionsAccountOwnershipId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FinancialConnectionsAccountOwnershipId);
