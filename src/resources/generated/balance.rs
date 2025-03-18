// ======================================
// This file was automatically generated.
// ======================================

use crate::params::Object;
use crate::resources::{BalanceAmountBySourceType, Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Balance".
///
/// For more details see <https://stripe.com/docs/api/balance/balance_object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Balance {
    /// Available funds that you can transfer or pay out automatically by Stripe or explicitly through the [Transfers API](https://stripe.com/docs/api#transfers) or [Payouts API](https://stripe.com/docs/api#payouts).
    ///
    /// You can find the available balance for each currency and payment type in the `source_types` property.
    pub available: Vec<BalanceAmount>,

    /// Funds held due to negative balances on connected Custom accounts.
    ///
    /// You can find the connect reserve balance for each currency and payment type in the `source_types` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_reserved: Option<Vec<BalanceAmount>>,

    /// Funds that you can pay out using Instant Payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_available: Option<Vec<BalanceAmountNet>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing: Option<BalanceDetail>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Funds that aren't available in the balance yet.
    ///
    /// You can find the pending balance for each currency and each payment type in the `source_types` property.
    pub pending: Vec<BalanceAmount>,
}

impl Object for Balance {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "balance"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BalanceAmountNet {
    /// Balance amount.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_types: Option<BalanceAmountBySourceType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BalanceDetail {
    /// Funds that are available for use.
    pub available: Vec<BalanceAmount>,
}
