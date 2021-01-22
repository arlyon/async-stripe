// ======================================
// This file was automatically generated.
// ======================================

use self::Currency;
use crate::params::Object;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Balance".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Balance {
    /// Funds that are available to be transferred or paid out, whether automatically by Stripe or explicitly via the [Transfers API](https://stripe.com/docs/api#transfers) or [Payouts API](https://stripe.com/docs/api#payouts).
    ///
    /// The available balance for each currency and payment type can be found in the `source_types` property.
    pub available: Vec<BalanceAmount>,

    /// Funds held due to negative balances on connected Custom accounts.
    ///
    /// The connect reserve balance for each currency and payment type can be found in the `source_types` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_reserved: Option<Vec<BalanceAmount>>,

    /// Funds that can be paid out using Instant Payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_available: Option<Vec<BalanceAmount>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing: Option<BalanceDetail>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Funds that are not yet available in the balance, due to the 7-day rolling pay cycle.
    ///
    /// The pending balance for each currency, and for each payment type, can be found in the `source_types` property.
    pub pending: Vec<BalanceAmount>,
}

impl Object for Balance {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "balance"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BalanceAmount {
    /// Balance amount.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_types: Option<BalanceAmountBySourceType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BalanceAmountBySourceType {
    /// Amount for bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<i64>,

    /// Amount for card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<i64>,

    /// Amount for FPX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BalanceDetail {
    /// Funds that are available for use.
    pub available: Vec<BalanceAmount>,
}
