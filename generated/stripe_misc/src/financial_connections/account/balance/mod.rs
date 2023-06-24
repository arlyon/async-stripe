#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Balance {
    /// The time that the external institution calculated this balance.
    ///
    /// Measured in seconds since the Unix epoch.
    pub as_of: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash:
        Option<stripe_misc::financial_connections::account::balance::cash_balance::CashBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit:
        Option<stripe_misc::financial_connections::account::balance::credit_balance::CreditBalance>,
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
    pub type_: BalanceType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Balance {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The `type` of the balance.
///
/// An additional hash is included on the balance with a name matching this value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BalanceType {
    Cash,
    Credit,
}

impl BalanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::Credit => "credit",
        }
    }
}

impl std::str::FromStr for BalanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cash" => Ok(Self::Cash),
            "credit" => Ok(Self::Credit),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BalanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BalanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BalanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for BalanceType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BalanceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<BalanceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BalanceType::from_str(s)?);
        Ok(())
    }
}
pub mod cash_balance;
pub use cash_balance::CashBalance;
pub mod credit_balance;
pub use credit_balance::CreditBalance;
