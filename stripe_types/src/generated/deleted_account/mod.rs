#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedAccount {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::account::AccountId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedAccountObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DeletedAccountObject {
    Account,
}

impl DeletedAccountObject {
    pub fn as_str(self) -> &'static str {
        use DeletedAccountObject::*;
        match self {
            Account => "account",
        }
    }
}

impl std::str::FromStr for DeletedAccountObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedAccountObject::*;
        match s {
            "account" => Ok(Account),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedAccountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DeletedAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DeletedAccountObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedAccountObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for DeletedAccountObject"))
    }
}
impl stripe_types::Object for DeletedAccount {
    type Id = stripe_types::account::AccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
