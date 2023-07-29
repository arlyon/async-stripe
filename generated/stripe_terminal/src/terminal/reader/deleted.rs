#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedReader {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal::reader::TerminalReaderId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedReaderObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedReaderObject {
    TerminalReader,
}

impl DeletedReaderObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TerminalReader => "terminal.reader",
        }
    }
}

impl std::str::FromStr for DeletedReaderObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "terminal.reader" => Ok(Self::TerminalReader),

            _ => Err(()),
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
impl serde::Serialize for DeletedReaderObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedReaderObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedReaderObject"))
    }
}
impl stripe_types::Object for DeletedReader {
    type Id = stripe_terminal::terminal::reader::TerminalReaderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
