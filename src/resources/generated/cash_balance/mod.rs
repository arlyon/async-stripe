/// A customer's `Cash balance` represents real funds.
///
/// Customers can add funds to their cash balance by sending a bank transfer.
/// These funds can be used for payment and can eventually be paid out to your bank account.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CashBalance {
    /// A hash of all cash balances available to this customer.
    ///
    /// You cannot delete a customer with any cash balances, even if the balance is 0.
    /// Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub available: Option<i64>,
    /// The ID of the customer whose cash balance this object represents.
    pub customer: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CashBalanceObject,
    pub settings: crate::cash_balance::balance_settings::BalanceSettings,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CashBalance {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CashBalanceObject {
    CashBalance,
}

impl CashBalanceObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CashBalance => "cash_balance",
        }
    }
}

impl AsRef<str> for CashBalanceObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CashBalanceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod balance_settings;
pub mod requests;
pub use balance_settings::BalanceSettings;
