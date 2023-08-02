#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Application {
    /// Unique identifier for the object.
    pub id: stripe_types::application::ApplicationId,
    /// The name of the application.
    pub name: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ApplicationObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ApplicationObject {
    Application,
}

impl ApplicationObject {
    pub fn as_str(self) -> &'static str {
        use ApplicationObject::*;
        match self {
            Application => "application",
        }
    }
}

impl std::str::FromStr for ApplicationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ApplicationObject::*;
        match s {
            "application" => Ok(Application),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ApplicationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApplicationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ApplicationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ApplicationObject"))
    }
}
impl stripe_types::Object for Application {
    type Id = stripe_types::application::ApplicationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ApplicationId, "ca_");
pub mod deleted;
pub use deleted::DeletedApplication;
