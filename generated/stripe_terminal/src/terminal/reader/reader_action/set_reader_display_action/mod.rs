/// Represents a reader action to set the reader display.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetReaderDisplayAction {
    /// Cart object to be displayed by the reader.
    pub cart: Option<
        stripe_terminal::terminal::reader::reader_action::set_reader_display_action::cart::Cart,
    >,
    /// Type of information to be displayed by the reader.
    #[serde(rename = "type")]
    pub type_: SetReaderDisplayActionType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetReaderDisplayAction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of information to be displayed by the reader.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetReaderDisplayActionType {
    Cart,
}

impl SetReaderDisplayActionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cart => "cart",
        }
    }
}

impl std::str::FromStr for SetReaderDisplayActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cart" => Ok(Self::Cart),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SetReaderDisplayActionType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetReaderDisplayActionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetReaderDisplayActionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetReaderDisplayActionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
pub mod cart;
pub use cart::Cart;
