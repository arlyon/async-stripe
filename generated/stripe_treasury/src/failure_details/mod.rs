#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FailureDetails {
    /// Reason for the failure.
    pub code: FailureDetailsCode,
}
/// Reason for the failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        use FailureDetailsCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            BankAccountRestricted => "bank_account_restricted",
            BankOwnershipChanged => "bank_ownership_changed",
            DebitNotAuthorized => "debit_not_authorized",
            IncorrectAccountHolderAddress => "incorrect_account_holder_address",
            IncorrectAccountHolderName => "incorrect_account_holder_name",
            IncorrectAccountHolderTaxId => "incorrect_account_holder_tax_id",
            InsufficientFunds => "insufficient_funds",
            InvalidAccountNumber => "invalid_account_number",
            InvalidCurrency => "invalid_currency",
            NoAccount => "no_account",
            Other => "other",
        }
    }
}

impl std::str::FromStr for FailureDetailsCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FailureDetailsCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_ownership_changed" => Ok(BankOwnershipChanged),
            "debit_not_authorized" => Ok(DebitNotAuthorized),
            "incorrect_account_holder_address" => Ok(IncorrectAccountHolderAddress),
            "incorrect_account_holder_name" => Ok(IncorrectAccountHolderName),
            "incorrect_account_holder_tax_id" => Ok(IncorrectAccountHolderTaxId),
            "insufficient_funds" => Ok(InsufficientFunds),
            "invalid_account_number" => Ok(InvalidAccountNumber),
            "invalid_currency" => Ok(InvalidCurrency),
            "no_account" => Ok(NoAccount),
            "other" => Ok(Other),
            _ => Err(()),
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
impl serde::Serialize for FailureDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FailureDetailsCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FailureDetailsCode"))
    }
}
