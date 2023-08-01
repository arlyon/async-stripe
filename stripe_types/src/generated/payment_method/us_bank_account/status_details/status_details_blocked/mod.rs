#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatusDetailsBlocked {
    /// The ACH network code that resulted in this block.
    pub network_code: Option<StatusDetailsBlockedNetworkCode>,
    /// The reason why this PaymentMethod's fingerprint has been blocked.
    pub reason: Option<StatusDetailsBlockedReason>,
}
/// The ACH network code that resulted in this block.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusDetailsBlockedNetworkCode {
    R02,
    R03,
    R04,
    R05,
    R07,
    R08,
    R10,
    R11,
    R16,
    R20,
    R29,
    R31,
}

impl StatusDetailsBlockedNetworkCode {
    pub fn as_str(self) -> &'static str {
        use StatusDetailsBlockedNetworkCode::*;
        match self {
            R02 => "R02",
            R03 => "R03",
            R04 => "R04",
            R05 => "R05",
            R07 => "R07",
            R08 => "R08",
            R10 => "R10",
            R11 => "R11",
            R16 => "R16",
            R20 => "R20",
            R29 => "R29",
            R31 => "R31",
        }
    }
}

impl std::str::FromStr for StatusDetailsBlockedNetworkCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StatusDetailsBlockedNetworkCode::*;
        match s {
            "R02" => Ok(R02),
            "R03" => Ok(R03),
            "R04" => Ok(R04),
            "R05" => Ok(R05),
            "R07" => Ok(R07),
            "R08" => Ok(R08),
            "R10" => Ok(R10),
            "R11" => Ok(R11),
            "R16" => Ok(R16),
            "R20" => Ok(R20),
            "R29" => Ok(R29),
            "R31" => Ok(R31),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for StatusDetailsBlockedNetworkCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for StatusDetailsBlockedNetworkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for StatusDetailsBlockedNetworkCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StatusDetailsBlockedNetworkCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for StatusDetailsBlockedNetworkCode")
        })
    }
}
/// The reason why this PaymentMethod's fingerprint has been blocked.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusDetailsBlockedReason {
    BankAccountClosed,
    BankAccountFrozen,
    BankAccountInvalidDetails,
    BankAccountRestricted,
    BankAccountUnusable,
    DebitNotAuthorized,
}

impl StatusDetailsBlockedReason {
    pub fn as_str(self) -> &'static str {
        use StatusDetailsBlockedReason::*;
        match self {
            BankAccountClosed => "bank_account_closed",
            BankAccountFrozen => "bank_account_frozen",
            BankAccountInvalidDetails => "bank_account_invalid_details",
            BankAccountRestricted => "bank_account_restricted",
            BankAccountUnusable => "bank_account_unusable",
            DebitNotAuthorized => "debit_not_authorized",
        }
    }
}

impl std::str::FromStr for StatusDetailsBlockedReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StatusDetailsBlockedReason::*;
        match s {
            "bank_account_closed" => Ok(BankAccountClosed),
            "bank_account_frozen" => Ok(BankAccountFrozen),
            "bank_account_invalid_details" => Ok(BankAccountInvalidDetails),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_account_unusable" => Ok(BankAccountUnusable),
            "debit_not_authorized" => Ok(DebitNotAuthorized),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for StatusDetailsBlockedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for StatusDetailsBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for StatusDetailsBlockedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StatusDetailsBlockedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for StatusDetailsBlockedReason"))
    }
}
