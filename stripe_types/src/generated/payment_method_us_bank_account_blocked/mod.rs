#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodUsBankAccountBlocked {
    /// The ACH network code that resulted in this block.
    pub network_code: Option<PaymentMethodUsBankAccountBlockedNetworkCode>,
    /// The reason why this PaymentMethod's fingerprint has been blocked.
    pub reason: Option<PaymentMethodUsBankAccountBlockedReason>,
}
/// The ACH network code that resulted in this block.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodUsBankAccountBlockedNetworkCode {
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

impl PaymentMethodUsBankAccountBlockedNetworkCode {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodUsBankAccountBlockedNetworkCode::*;
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

impl std::str::FromStr for PaymentMethodUsBankAccountBlockedNetworkCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountBlockedNetworkCode::*;
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

impl AsRef<str> for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodUsBankAccountBlockedNetworkCode"))
    }
}
/// The reason why this PaymentMethod's fingerprint has been blocked.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodUsBankAccountBlockedReason {
    BankAccountClosed,
    BankAccountFrozen,
    BankAccountInvalidDetails,
    BankAccountRestricted,
    BankAccountUnusable,
    DebitNotAuthorized,
}

impl PaymentMethodUsBankAccountBlockedReason {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodUsBankAccountBlockedReason::*;
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

impl std::str::FromStr for PaymentMethodUsBankAccountBlockedReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountBlockedReason::*;
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

impl AsRef<str> for PaymentMethodUsBankAccountBlockedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodUsBankAccountBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodUsBankAccountBlockedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountBlockedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodUsBankAccountBlockedReason"))
    }
}
