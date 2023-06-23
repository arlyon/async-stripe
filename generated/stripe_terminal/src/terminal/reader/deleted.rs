#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedReader {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal::reader::TerminalReaderId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedReaderObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedReader {
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
pub enum DeletedReaderObject {
    #[serde(rename = "terminal.reader")]
    TerminalReader,
}

impl DeletedReaderObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TerminalReader => "terminal.reader",
        }
    }
}

impl AsRef<str> for DeletedReaderObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedReaderObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for DeletedReader {
    type Id = stripe_terminal::terminal::reader::TerminalReaderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
