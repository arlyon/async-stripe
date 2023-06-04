#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReturnedDetails {
    /// Reason for the return.
    pub code: ReturnedDetailsCode,
    /// The Transaction associated with this object.
    pub transaction: crate::Expandable<crate::treasury::transaction::Transaction>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReturnedDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Reason for the return.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ReturnedDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    Declined,
    IncorrectAccountHolderName,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}

impl ReturnedDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::BankAccountRestricted => "bank_account_restricted",
            Self::BankOwnershipChanged => "bank_ownership_changed",
            Self::Declined => "declined",
            Self::IncorrectAccountHolderName => "incorrect_account_holder_name",
            Self::InvalidAccountNumber => "invalid_account_number",
            Self::InvalidCurrency => "invalid_currency",
            Self::NoAccount => "no_account",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for ReturnedDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
