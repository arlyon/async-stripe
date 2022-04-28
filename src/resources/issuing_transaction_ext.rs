use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `IssuingTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTransactionType {
    Capture,
    CashWithdrawal,
    Dispute,
    DisputeLoss,
    Refund,
    RefundReversal,
}

impl IssuingTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingTransactionType::Capture => "capture",
            IssuingTransactionType::CashWithdrawal => "cash_withdrawal",
            IssuingTransactionType::Dispute => "dispute",
            IssuingTransactionType::DisputeLoss => "dispute_loss",
            IssuingTransactionType::Refund => "refund",
            IssuingTransactionType::RefundReversal => "refund_reversal",
        }
    }
}

impl AsRef<str> for IssuingTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for IssuingTransactionType {
    fn default() -> Self {
        Self::Capture
    }
}
