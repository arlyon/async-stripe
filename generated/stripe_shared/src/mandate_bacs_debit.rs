#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MandateBacsDebit {
    /// The status of the mandate on the Bacs network.
    /// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: MandateBacsDebitNetworkStatus,
    /// The unique reference identifying the mandate on the Bacs network.
    pub reference: String,
    /// When the mandate is revoked on the Bacs network this field displays the reason for the revocation.
    pub revocation_reason: Option<MandateBacsDebitRevocationReason>,
    /// The URL that will contain the mandate that the customer has signed.
    pub url: String,
}
/// The status of the mandate on the Bacs network.
/// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateBacsDebitNetworkStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}
impl MandateBacsDebitNetworkStatus {
    pub fn as_str(self) -> &'static str {
        use MandateBacsDebitNetworkStatus::*;
        match self {
            Accepted => "accepted",
            Pending => "pending",
            Refused => "refused",
            Revoked => "revoked",
        }
    }
}

impl std::str::FromStr for MandateBacsDebitNetworkStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateBacsDebitNetworkStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "pending" => Ok(Pending),
            "refused" => Ok(Refused),
            "revoked" => Ok(Revoked),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for MandateBacsDebitNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateBacsDebitNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateBacsDebitNetworkStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateBacsDebitNetworkStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for MandateBacsDebitNetworkStatus")
        })
    }
}
/// When the mandate is revoked on the Bacs network this field displays the reason for the revocation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateBacsDebitRevocationReason {
    AccountClosed,
    BankAccountRestricted,
    BankOwnershipChanged,
    CouldNotProcess,
    DebitNotAuthorized,
}
impl MandateBacsDebitRevocationReason {
    pub fn as_str(self) -> &'static str {
        use MandateBacsDebitRevocationReason::*;
        match self {
            AccountClosed => "account_closed",
            BankAccountRestricted => "bank_account_restricted",
            BankOwnershipChanged => "bank_ownership_changed",
            CouldNotProcess => "could_not_process",
            DebitNotAuthorized => "debit_not_authorized",
        }
    }
}

impl std::str::FromStr for MandateBacsDebitRevocationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateBacsDebitRevocationReason::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_ownership_changed" => Ok(BankOwnershipChanged),
            "could_not_process" => Ok(CouldNotProcess),
            "debit_not_authorized" => Ok(DebitNotAuthorized),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for MandateBacsDebitRevocationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateBacsDebitRevocationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateBacsDebitRevocationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateBacsDebitRevocationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for MandateBacsDebitRevocationReason")
        })
    }
}
