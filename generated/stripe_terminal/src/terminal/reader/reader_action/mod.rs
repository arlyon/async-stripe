/// Represents an action performed by the reader.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
pub set_reader_display: Option<stripe_terminal::terminal::reader::reader_action::set_reader_display_action::SetReaderDisplayAction>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ReaderActionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "failed" => Ok(Self::Failed),
            "in_progress" => Ok(Self::InProgress),
            "succeeded" => Ok(Self::Succeeded),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReaderActionStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReaderActionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ReaderActionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReaderActionStatus::from_str(s)?);
        Ok(())
    }
}
/// Type of action performed by the reader.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ReaderActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "process_payment_intent" => Ok(Self::ProcessPaymentIntent),
            "process_setup_intent" => Ok(Self::ProcessSetupIntent),
            "set_reader_display" => Ok(Self::SetReaderDisplay),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReaderActionType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReaderActionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ReaderActionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReaderActionType::from_str(s)?);
        Ok(())
    }
}
pub mod process_payment_intent_action;
pub use process_payment_intent_action::ProcessPaymentIntentAction;
pub mod process_setup_intent_action;
pub use process_setup_intent_action::ProcessSetupIntentAction;
pub mod set_reader_display_action;
pub use set_reader_display_action::SetReaderDisplayAction;
