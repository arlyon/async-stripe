#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedPerson {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::person::PersonId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedPersonObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedPersonObject {
    Person,
}

impl DeletedPersonObject {
    pub fn as_str(self) -> &'static str {
        use DeletedPersonObject::*;
        match self {
            Person => "person",
        }
    }
}

impl std::str::FromStr for DeletedPersonObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedPersonObject::*;
        match s {
            "person" => Ok(Person),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedPersonObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedPersonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedPersonObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedPersonObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedPersonObject"))
    }
}
impl stripe_types::Object for DeletedPerson {
    type Id = stripe_types::person::PersonId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
