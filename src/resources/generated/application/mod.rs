#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Application {
    /// Unique identifier for the object.
    pub id: crate::application::ApplicationId,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for Application {
    type Id = crate::application::ApplicationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(ApplicationId, "ca_");
pub mod deleted;
pub use deleted::DeletedApplication;
