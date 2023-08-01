/// Represents an action performed by the reader.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReaderAction {
    /// Failure code, only set if status is `failed`.
pub failure_code: Option<String>,
    /// Detailed failure message, only set if status is `failed`.
pub failure_message: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
pub process_payment_intent: Option<stripe_terminal::terminal::reader::reader_action::process_payment_intent_action::ProcessPaymentIntentAction>,
#[serde(skip_serializing_if = "Option::is_none")]
pub process_setup_intent: Option<stripe_terminal::terminal::reader::reader_action::process_setup_intent_action::ProcessSetupIntentAction>,
#[serde(skip_serializing_if = "Option::is_none")]
pub refund_payment: Option<stripe_terminal::terminal::reader::reader_action::refund_payment_action::RefundPaymentAction>,
#[serde(skip_serializing_if = "Option::is_none")]
pub set_reader_display: Option<stripe_terminal::terminal::reader::reader_action::set_reader_display_action::SetReaderDisplayAction>,
    /// Status of the action performed by the reader.
pub status: ReaderActionStatus,
    /// Type of action performed by the reader.
#[serde(rename = "type")]
pub type_: ReaderActionType,

}
/// Status of the action performed by the reader.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
}

impl ReaderActionStatus {
    pub fn as_str(self) -> &'static str {
        use ReaderActionStatus::*;
        match self {
            Failed => "failed",
            InProgress => "in_progress",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for ReaderActionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReaderActionStatus::*;
        match s {
            "failed" => Ok(Failed),
            "in_progress" => Ok(InProgress),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
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
impl serde::Serialize for ReaderActionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReaderActionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReaderActionStatus"))
    }
}
/// Type of action performed by the reader.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReaderActionType {
    ProcessPaymentIntent,
    ProcessSetupIntent,
    RefundPayment,
    SetReaderDisplay,
}

impl ReaderActionType {
    pub fn as_str(self) -> &'static str {
        use ReaderActionType::*;
        match self {
            ProcessPaymentIntent => "process_payment_intent",
            ProcessSetupIntent => "process_setup_intent",
            RefundPayment => "refund_payment",
            SetReaderDisplay => "set_reader_display",
        }
    }
}

impl std::str::FromStr for ReaderActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReaderActionType::*;
        match s {
            "process_payment_intent" => Ok(ProcessPaymentIntent),
            "process_setup_intent" => Ok(ProcessSetupIntent),
            "refund_payment" => Ok(RefundPayment),
            "set_reader_display" => Ok(SetReaderDisplay),
            _ => Err(()),
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
impl serde::Serialize for ReaderActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReaderActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReaderActionType"))
    }
}
pub mod process_payment_intent_action;
pub use process_payment_intent_action::ProcessPaymentIntentAction;
pub mod process_setup_intent_action;
pub use process_setup_intent_action::ProcessSetupIntentAction;
pub mod refund_payment_action;
pub use refund_payment_action::RefundPaymentAction;
pub mod set_reader_display_action;
pub use set_reader_display_action::SetReaderDisplayAction;
