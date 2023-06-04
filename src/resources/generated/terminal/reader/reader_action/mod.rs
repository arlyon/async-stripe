/// Represents an action performed by the reader.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReaderAction {
    /// Failure code, only set if status is `failed`.
pub failure_code: Option<String>,
    /// Detailed failure message, only set if status is `failed`.
pub failure_message: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
pub process_payment_intent: Option<crate::terminal::reader::reader_action::process_payment_intent_action::ProcessPaymentIntentAction>,
#[serde(skip_serializing_if = "Option::is_none")]
pub process_setup_intent: Option<crate::terminal::reader::reader_action::process_setup_intent_action::ProcessSetupIntentAction>,
#[serde(skip_serializing_if = "Option::is_none")]
pub set_reader_display: Option<crate::terminal::reader::reader_action::set_reader_display_action::SetReaderDisplayAction>,
    /// Status of the action performed by the reader.
pub status: ReaderActionStatus,
    /// Type of action performed by the reader.
#[serde(rename = "type")]
pub type_: ReaderActionType,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReaderAction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Status of the action performed by the reader.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
}

impl ReaderActionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::InProgress => "in_progress",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for ReaderActionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Type of action performed by the reader.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ReaderActionType {
    ProcessPaymentIntent,
    ProcessSetupIntent,
    SetReaderDisplay,
}

impl ReaderActionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProcessPaymentIntent => "process_payment_intent",
            Self::ProcessSetupIntent => "process_setup_intent",
            Self::SetReaderDisplay => "set_reader_display",
        }
    }
}

impl AsRef<str> for ReaderActionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod process_payment_intent_action;
pub use process_payment_intent_action::ProcessPaymentIntentAction;
pub mod process_setup_intent_action;
pub use process_setup_intent_action::ProcessSetupIntentAction;
pub mod set_reader_display_action;
pub use set_reader_display_action::SetReaderDisplayAction;
