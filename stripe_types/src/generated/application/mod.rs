#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Application {
    /// Unique identifier for the object.
    pub id: stripe_types::ApplicationId,
    /// The name of the application.
    pub name: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ApplicationObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Application {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
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
        match self {
            Self::Application => "application",
        }
    }
}

impl std::str::FromStr for ApplicationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application" => Ok(Self::Application),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ApplicationObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ApplicationObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ApplicationObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ApplicationObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for Application {
    type Id = stripe_types::ApplicationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
pub mod deleted;
pub use deleted::DeletedApplication;
