// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{PayoutId, SourceId};
use crate::params::{Expand, List, Object, RangeQuery, Timestamp};
use crate::resources::Currency;

/// The resource representing a Stripe "Balance".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Balance {
    /// Funds that are available to be transferred or paid out, whether automatically by Stripe or explicitly via the [Transfers API](https://stripe.com/docs/api#transfers) or [Payouts API](https://stripe.com/docs/api#payouts).
    ///
    /// The available balance for each currency and payment type can be found in the `source_types` property.
    pub available: Vec<BalanceAmount>,

    /// Funds held due to negative balances on connected Custom accounts.
    ///
    /// The connect reserve balance for each currency and payment type can be found in the `source_types` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_reserved: Option<Box<Vec<BalanceAmount>>>,

    /// Funds that can be paid out using Instant Payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_available: Option<Box<Vec<BalanceAmount>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing: Option<Box<BalanceDetail>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Funds that are not yet available in the balance, due to the 7-day rolling pay cycle.
    ///
    /// The pending balance for each currency, and for each payment type, can be found in the `source_types` property.
    pub pending: Vec<BalanceAmount>,
}

impl Balance {
    /// Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth).
    ///
    /// The transactions are returned in sorted order, with the most recent transactions appearing first.  Note that this endpoint was previously called “Balance history” and used the path `/v1/balance/history`.
    pub fn list(client: &Client, params: ListBalances<'_>) -> Response<List<Balance>> {
        client.get_query("/balance/history", &params)
    }
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
    pub source_types: Option<Box<BalanceAmountBySourceType>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BalanceAmountBySourceType {
    /// Amount for bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<Box<i64>>,

    /// Amount for card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<i64>>,

    /// Amount for FPX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<Box<i64>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BalanceDetail {
    /// Funds that are available for use.
    pub available: Vec<BalanceAmount>,
}

/// The parameters for `Balance::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListBalances<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return transactions in a certain currency.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// For automatic Stripe payouts only, only returns transactions that were paid out on the specified payout ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<PayoutId>,

    /// Only returns the original transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,

    /// Only returns transactions of the given type.
    ///
    /// One of: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
}

impl<'a> ListBalances<'a> {
    pub fn new() -> Self {
        ListBalances {
            created: Default::default(),
            currency: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payout: Default::default(),
            source: Default::default(),
            starting_after: Default::default(),
            type_: Default::default(),
        }
    }
}
