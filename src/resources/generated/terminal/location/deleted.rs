#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedLocation {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: crate::terminal::location::TerminalLocationId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedLocationObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedLocation {
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
pub enum DeletedLocationObject {
    #[serde(rename = "terminal.location")]
    TerminalLocation,
}

impl DeletedLocationObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TerminalLocation => "terminal.location",
        }
    }
}

impl AsRef<str> for DeletedLocationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedLocationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for DeletedLocation {
    type Id = crate::terminal::location::TerminalLocationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
