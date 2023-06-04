#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FailureDetails {
    /// Reason for the failure.
    pub code: FailureDetailsCode,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FailureDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Reason for the failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FailureDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    DebitNotAuthorized,
    IncorrectAccountHolderAddress,
    IncorrectAccountHolderName,
    IncorrectAccountHolderTaxId,
    InsufficientFunds,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}

impl FailureDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::BankAccountRestricted => "bank_account_restricted",
            Self::BankOwnershipChanged => "bank_ownership_changed",
            Self::DebitNotAuthorized => "debit_not_authorized",
            Self::IncorrectAccountHolderAddress => "incorrect_account_holder_address",
            Self::IncorrectAccountHolderName => "incorrect_account_holder_name",
            Self::IncorrectAccountHolderTaxId => "incorrect_account_holder_tax_id",
            Self::InsufficientFunds => "insufficient_funds",
            Self::InvalidAccountNumber => "invalid_account_number",
            Self::InvalidCurrency => "invalid_currency",
            Self::NoAccount => "no_account",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for FailureDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
