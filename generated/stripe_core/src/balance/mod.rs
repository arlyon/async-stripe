/// This is an object representing your Stripe balance.
///
/// You can retrieve it to see the balance currently on your Stripe account.  You can also retrieve the balance history, which contains a list of [transactions](https://stripe.com/docs/reporting/balance-transaction-types) that contributed to the balance (charges, payouts, and so forth).  The available and pending amounts for each currency are broken down further by payment source types.  Related guide: [Understanding Connect account balances](https://stripe.com/docs/connect/account-balances).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Balance {
    /// Funds that are available to be transferred or paid out, whether automatically by Stripe or explicitly via the [Transfers API](https://stripe.com/docs/api#transfers) or [Payouts API](https://stripe.com/docs/api#payouts).
    ///
    /// The available balance for each currency and payment type can be found in the `source_types` property.
    pub available: Vec<stripe_core::balance::money::Money>,
    /// Funds held due to negative balances on connected Custom accounts.
    ///
    /// The connect reserve balance for each currency and payment type can be found in the `source_types` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_reserved: Option<Vec<stripe_core::balance::money::Money>>,
    /// Funds that can be paid out using Instant Payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_available: Option<Vec<stripe_core::balance::money::Money>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing: Option<stripe_core::balance::details::Details>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: BalanceObject,
    /// Funds that are not yet available in the balance, due to the 7-day rolling pay cycle.
    ///
    /// The pending balance for each currency, and for each payment type, can be found in the `source_types` property.
    pub pending: Vec<stripe_core::balance::money::Money>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BalanceObject {
    Balance,
}

impl BalanceObject {
    pub fn as_str(self) -> &'static str {
        use BalanceObject::*;
        match self {
            Balance => "balance",
        }
    }
}

impl std::str::FromStr for BalanceObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BalanceObject::*;
        match s {
            "balance" => Ok(Balance),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BalanceObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BalanceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BalanceObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BalanceObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for BalanceObject"))
    }
}
pub mod money;
pub use money::Money;
pub mod details;
pub use details::Details;
pub mod requests;
