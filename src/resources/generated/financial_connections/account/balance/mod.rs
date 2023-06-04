#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Balance {
    /// The time that the external institution calculated this balance.
    ///
    /// Measured in seconds since the Unix epoch.
    pub as_of: crate::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash: Option<crate::financial_connections::account::balance::cash_balance::CashBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit:
        Option<crate::financial_connections::account::balance::credit_balance::CreditBalance>,
    /// The balances owed to (or by) the account holder.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    ///
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub current: i64,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
pub mod cash_balance;
pub use cash_balance::CashBalance;
pub mod credit_balance;
pub use credit_balance::CreditBalance;
