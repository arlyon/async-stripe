#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedApplication {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::ApplicationId,
    /// The name of the application.
    pub name: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedApplicationObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedApplicationObject {
    Application,
}

impl DeletedApplicationObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Application => "application",
        }
    }
}

impl std::str::FromStr for DeletedApplicationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application" => Ok(Self::Application),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedApplicationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedApplicationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedApplicationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedApplicationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedApplicationObject"))
    }
}
impl stripe_types::Object for DeletedApplication {
    type Id = stripe_types::ApplicationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
