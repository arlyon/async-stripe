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

impl std::str::FromStr for FailureDetailsCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account_closed" => Ok(Self::AccountClosed),
            "account_frozen" => Ok(Self::AccountFrozen),
            "bank_account_restricted" => Ok(Self::BankAccountRestricted),
            "bank_ownership_changed" => Ok(Self::BankOwnershipChanged),
            "debit_not_authorized" => Ok(Self::DebitNotAuthorized),
            "incorrect_account_holder_address" => Ok(Self::IncorrectAccountHolderAddress),
            "incorrect_account_holder_name" => Ok(Self::IncorrectAccountHolderName),
            "incorrect_account_holder_tax_id" => Ok(Self::IncorrectAccountHolderTaxId),
            "insufficient_funds" => Ok(Self::InsufficientFunds),
            "invalid_account_number" => Ok(Self::InvalidAccountNumber),
            "invalid_currency" => Ok(Self::InvalidCurrency),
            "no_account" => Ok(Self::NoAccount),
            "other" => Ok(Self::Other),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FailureDetailsCode"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FailureDetailsCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<FailureDetailsCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FailureDetailsCode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
