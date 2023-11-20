/// This is an object representing your Stripe balance.
///
/// You can retrieve it to see the balance currently on your Stripe account.  You can also retrieve the balance history, which contains a list of [transactions](https://stripe.com/docs/reporting/balance-transaction-types) that contributed to the balance (charges, payouts, and so forth).  The available and pending amounts for each currency are broken down further by payment source types.  Related guide: [Understanding Connect account balances](https://stripe.com/docs/connect/account-balances).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Balance {
    /// Available funds that you can transfer or pay out automatically by Stripe or explicitly through the [Transfers API](https://stripe.com/docs/api#transfers) or [Payouts API](https://stripe.com/docs/api#payouts).
    ///
    /// You can find the available balance for each currency and payment type in the `source_types` property.
    pub available: Vec<stripe_core::BalanceAmount>,
    /// Funds held due to negative balances on connected Custom accounts.
    ///
    /// You can find the connect reserve balance for each currency and payment type in the `source_types` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_reserved: Option<Vec<stripe_core::BalanceAmount>>,
    /// Funds that you can pay out using Instant Payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_available: Option<Vec<stripe_core::BalanceAmountNet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing: Option<stripe_core::BalanceDetail>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Funds that aren't available in the balance yet.
    ///
    /// You can find the pending balance for each currency and each payment type in the `source_types` property.
    pub pending: Vec<stripe_core::BalanceAmount>,
}
#[cfg(feature = "balance")]
mod requests;
#[cfg(feature = "balance")]
pub use requests::*;
