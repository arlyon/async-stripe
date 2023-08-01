/// Represents a reader action to set the reader display.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetReaderDisplayAction {
    /// Cart object to be displayed by the reader.
    pub cart: Option<
        stripe_terminal::terminal::reader::reader_action::set_reader_display_action::cart::Cart,
    >,
    /// Type of information to be displayed by the reader.
    #[serde(rename = "type")]
    pub type_: SetReaderDisplayActionType,
}
/// Type of information to be displayed by the reader.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetReaderDisplayActionType {
    Cart,
}

impl SetReaderDisplayActionType {
    pub fn as_str(self) -> &'static str {
        use SetReaderDisplayActionType::*;
        match self {
            Cart => "cart",
        }
    }
}

impl std::str::FromStr for SetReaderDisplayActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetReaderDisplayActionType::*;
        match s {
            "cart" => Ok(Cart),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetReaderDisplayActionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SetReaderDisplayActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetReaderDisplayActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SetReaderDisplayActionType"))
    }
}
pub mod cart;
pub use cart::Cart;
