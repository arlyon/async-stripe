use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `IssuingDispute`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeReason {
    Fraudulent,
    Other,
}

impl IssuingDisputeReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeReason::Fraudulent => "fraudulent",
            IssuingDisputeReason::Other => "other",
        }
    }
}

impl AsRef<str> for IssuingDisputeReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingDispute`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeStatus {
    Lost,
    UnderReview,
    Unsubmitted,
    Won,
}

impl IssuingDisputeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeStatus::Lost => "lost",
            IssuingDisputeStatus::UnderReview => "under_review",
            IssuingDisputeStatus::Unsubmitted => "unsubmitted",
            IssuingDisputeStatus::Won => "won",
        }
    }
}

impl AsRef<str> for IssuingDisputeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for IssuingDisputeStatus {
    fn default() -> Self {
        Self::Unsubmitted
    }
}
