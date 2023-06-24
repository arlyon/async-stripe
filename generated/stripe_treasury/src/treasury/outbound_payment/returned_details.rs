#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReturnedDetails {
    /// Reason for the return.
    pub code: ReturnedDetailsCode,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::treasury::transaction::Transaction>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReturnedDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Reason for the return.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ReturnedDetailsCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account_closed" => Ok(Self::AccountClosed),
            "account_frozen" => Ok(Self::AccountFrozen),
            "bank_account_restricted" => Ok(Self::BankAccountRestricted),
            "bank_ownership_changed" => Ok(Self::BankOwnershipChanged),
            "declined" => Ok(Self::Declined),
            "incorrect_account_holder_name" => Ok(Self::IncorrectAccountHolderName),
            "invalid_account_number" => Ok(Self::InvalidAccountNumber),
            "invalid_currency" => Ok(Self::InvalidCurrency),
            "no_account" => Ok(Self::NoAccount),
            "other" => Ok(Self::Other),

            _ => Err(()),
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
impl serde::Serialize for ReturnedDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReturnedDetailsCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReturnedDetailsCode"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReturnedDetailsCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ReturnedDetailsCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReturnedDetailsCode::from_str(s)?);
        Ok(())
    }
}
