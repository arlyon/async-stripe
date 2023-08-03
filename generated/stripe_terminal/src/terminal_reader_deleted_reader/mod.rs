#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalReaderDeletedReader {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_reader_reader::TerminalReaderId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TerminalReaderDeletedReaderObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderDeletedReaderObject {
    TerminalReader,
}

impl TerminalReaderDeletedReaderObject {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderDeletedReaderObject::*;
        match self {
            TerminalReader => "terminal.reader",
        }
    }
}

impl std::str::FromStr for TerminalReaderDeletedReaderObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderDeletedReaderObject::*;
        match s {
            "terminal.reader" => Ok(TerminalReader),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TerminalReaderDeletedReaderObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderDeletedReaderObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderDeletedReaderObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalReaderDeletedReaderObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalReaderDeletedReaderObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TerminalReaderDeletedReaderObject"))
    }
}
impl stripe_types::Object for TerminalReaderDeletedReader {
    type Id = stripe_terminal::terminal_reader_reader::TerminalReaderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
