
/// Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth).
///
/// The transactions are returned in sorted order, with the most recent transactions appearing first.  Note that this endpoint was previously called “Balance history” and used the path `/v1/balance/history`.
pub fn list(client: &stripe::Client, params: ListBalanceTransaction) -> stripe::Response<stripe_types::List<stripe_types::BalanceTransaction>> {
    client.get_query("/balance_transactions", params)
}
/// Retrieves the balance transaction with the given ID.
///
/// Note that this endpoint previously used the path `/v1/balance/history/:id`.
pub fn retrieve(client: &stripe::Client, id: &stripe_types::balance_transaction::BalanceTransactionId, params: RetrieveBalanceTransaction) -> stripe::Response<stripe_types::BalanceTransaction> {
    client.get_query(&format!("/balance_transactions/{id}", id = id), params)
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListBalanceTransaction<'a> {
    /// This parameter is deprecated and we recommend listing by created and filtering in memory instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_on: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return transactions in a certain currency.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// For automatic Stripe payouts only, only returns transactions that were paid out on the specified payout ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<&'a str>,
    /// Only returns the original transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only returns transactions of the given type.
    ///
    /// One of: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
}
impl<'a> ListBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveBalanceTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
