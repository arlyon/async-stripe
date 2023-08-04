/// Represents an action performed by the reader.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalReaderReaderResourceReaderAction {
    /// Failure code, only set if status is `failed`.
    pub failure_code: Option<String>,
    /// Detailed failure message, only set if status is `failed`.
    pub failure_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_payment_intent:
        Option<stripe_terminal::TerminalReaderReaderResourceProcessPaymentIntentAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_setup_intent:
        Option<stripe_terminal::TerminalReaderReaderResourceProcessSetupIntentAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_payment: Option<stripe_terminal::TerminalReaderReaderResourceRefundPaymentAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_reader_display:
        Option<stripe_terminal::TerminalReaderReaderResourceSetReaderDisplayAction>,
    /// Status of the action performed by the reader.
    pub status: TerminalReaderReaderResourceReaderActionStatus,
    /// Type of action performed by the reader.
    #[serde(rename = "type")]
    pub type_: TerminalReaderReaderResourceReaderActionType,
}
/// Status of the action performed by the reader.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
}

impl TerminalReaderReaderResourceReaderActionStatus {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceReaderActionStatus::*;
        match self {
            Failed => "failed",
            InProgress => "in_progress",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceReaderActionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceReaderActionStatus::*;
        match s {
            "failed" => Ok(Failed),
            "in_progress" => Ok(InProgress),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TerminalReaderReaderResourceReaderActionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalReaderReaderResourceReaderActionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceReaderActionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TerminalReaderReaderResourceReaderActionStatus",
            )
        })
    }
}
/// Type of action performed by the reader.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceReaderActionType {
    ProcessPaymentIntent,
    ProcessSetupIntent,
    RefundPayment,
    SetReaderDisplay,
}

impl TerminalReaderReaderResourceReaderActionType {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceReaderActionType::*;
        match self {
            ProcessPaymentIntent => "process_payment_intent",
            ProcessSetupIntent => "process_setup_intent",
            RefundPayment => "refund_payment",
            SetReaderDisplay => "set_reader_display",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceReaderActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceReaderActionType::*;
        match s {
            "process_payment_intent" => Ok(ProcessPaymentIntent),
            "process_setup_intent" => Ok(ProcessSetupIntent),
            "refund_payment" => Ok(RefundPayment),
            "set_reader_display" => Ok(SetReaderDisplay),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TerminalReaderReaderResourceReaderActionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalReaderReaderResourceReaderActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceReaderActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TerminalReaderReaderResourceReaderActionType",
            )
        })
    }
}
