#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceBalance {
    /// The time that the external institution calculated this balance.
    ///
    /// Measured in seconds since the Unix epoch.
    pub as_of: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash: Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCashBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCreditBalance>,
    /// The balances owed to (or by) the account holder.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    ///
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub current: std::collections::HashMap<String, i64>,
    /// The `type` of the balance.
    ///
    /// An additional hash is included on the balance with a name matching this value.
    #[serde(rename = "type")]
    pub type_: BankConnectionsResourceBalanceType,
}
/// The `type` of the balance.
///
/// An additional hash is included on the balance with a name matching this value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceBalanceType {
    Cash,
    Credit,
}

impl BankConnectionsResourceBalanceType {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceBalanceType::*;
        match self {
            Cash => "cash",
            Credit => "credit",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceBalanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceBalanceType::*;
        match s {
            "cash" => Ok(Cash),
            "credit" => Ok(Credit),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankConnectionsResourceBalanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceBalanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceBalanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BankConnectionsResourceBalanceType")
        })
    }
}
