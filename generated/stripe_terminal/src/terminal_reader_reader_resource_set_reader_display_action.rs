/// Represents a reader action to set the reader display
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalReaderReaderResourceSetReaderDisplayAction {
    /// Cart object to be displayed by the reader.
    pub cart: Option<stripe_terminal::TerminalReaderReaderResourceCart>,
    /// Type of information to be displayed by the reader.
    #[serde(rename = "type")]
    pub type_: TerminalReaderReaderResourceSetReaderDisplayActionType,
}
/// Type of information to be displayed by the reader.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceSetReaderDisplayActionType {
    Cart,
}
impl TerminalReaderReaderResourceSetReaderDisplayActionType {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceSetReaderDisplayActionType::*;
        match self {
            Cart => "cart",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceSetReaderDisplayActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceSetReaderDisplayActionType::*;
        match s {
            "cart" => Ok(Cart),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TerminalReaderReaderResourceSetReaderDisplayActionType",
            )
        })
    }
}
