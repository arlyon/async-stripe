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
    pub available: Option<std::collections::HashMap<String, i64>>,
    /// The ID of the customer whose cash balance this object represents.
    pub customer: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CashBalanceObject,
    pub settings: stripe_core::cash_balance::balance_settings::BalanceSettings,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CashBalanceObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cash_balance" => Ok(Self::CashBalance),

            _ => Err(()),
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
impl serde::Serialize for CashBalanceObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CashBalanceObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CashBalanceObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CashBalanceObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CashBalanceObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CashBalanceObject::from_str(s)?);
        Ok(())
    }
}
pub mod balance_settings;
pub mod requests;
pub use balance_settings::BalanceSettings;
