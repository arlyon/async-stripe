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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
pub mod cart;
pub use cart::Cart;
