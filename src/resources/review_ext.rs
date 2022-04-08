use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `Review`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewReason {
    Approved,
    Disputed,
    Manual,
    Refunded,
    RefundedAsFraud,
    Rule,
}

impl ReviewReason {
    pub fn as_str(self) -> &'static str {
        match self {
            ReviewReason::Approved => "approved",
            ReviewReason::Disputed => "disputed",
            ReviewReason::Manual => "manual",
            ReviewReason::Refunded => "refunded",
            ReviewReason::RefundedAsFraud => "refunded_as_fraud",
            ReviewReason::Rule => "rule",
        }
    }
}

impl AsRef<str> for ReviewReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReviewReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for ReviewReason {
    fn default() -> Self {
        Self::Approved
    }
}
