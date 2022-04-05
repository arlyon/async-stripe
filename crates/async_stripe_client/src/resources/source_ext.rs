use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `Source`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceStatus {
    Canceled,
    Chargeable,
    Consumed,
    Failed,
    Pending,
}

impl SourceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceStatus::Canceled => "canceled",
            SourceStatus::Chargeable => "chargeable",
            SourceStatus::Consumed => "consumed",
            SourceStatus::Failed => "failed",
            SourceStatus::Pending => "pending",
        }
    }
}

impl AsRef<str> for SourceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for SourceStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// An enum representing the possible values of an `Source`'s `usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceUsage {
    Reusable,
    SingleUse,
}

impl SourceUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceUsage::Reusable => "reusable",
            SourceUsage::SingleUse => "single_use",
        }
    }
}

impl AsRef<str> for SourceUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SourceRedirectFlow`'s `failure_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceRedirectFlowFailureReason {
    Declined,
    ProcessingError,
    UserAbort,
}

impl SourceRedirectFlowFailureReason {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceRedirectFlowFailureReason::Declined => "declined",
            SourceRedirectFlowFailureReason::ProcessingError => "processing_error",
            SourceRedirectFlowFailureReason::UserAbort => "user_abort",
        }
    }
}

impl AsRef<str> for SourceRedirectFlowFailureReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceRedirectFlowFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for SourceRedirectFlowFailureReason {
    fn default() -> Self {
        Self::Declined
    }
}

/// An enum representing the possible values of an `SourceRedirectFlow`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceRedirectFlowStatus {
    Failed,
    NotRequired,
    Pending,
    Succeeded,
}

impl SourceRedirectFlowStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceRedirectFlowStatus::Failed => "failed",
            SourceRedirectFlowStatus::NotRequired => "not_required",
            SourceRedirectFlowStatus::Pending => "pending",
            SourceRedirectFlowStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for SourceRedirectFlowStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceRedirectFlowStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for SourceRedirectFlowStatus {
    fn default() -> Self {
        Self::Pending
    }
}
