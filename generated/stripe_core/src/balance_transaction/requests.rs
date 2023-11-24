#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListBalanceTransaction<'a> {
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
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
    /// Only returns transactions of the given type.
    ///
    /// One of: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `obligation_inbound`, `obligation_outbound`, `obligation_reversal_inbound`, `obligation_reversal_outbound`, `obligation_payout`, `obligation_payout_failure`, `payment`, `payment_failure_refund`, `payment_refund`, `payment_reversal`, `payment_unreconciled`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
}
impl<'a> ListBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListBalanceTransaction<'a> {
    /// Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth).
    ///
    /// The transactions are returned in sorted order, with the most recent transactions appearing first.  Note that this endpoint was previously called “Balance history” and used the path `/v1/balance/history`.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::BalanceTransaction>> {
        client.get_query("/balance_transactions", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::BalanceTransaction> {
        stripe::ListPaginator::from_params("/balance_transactions", self)
    }
}
impl<'a> stripe::PaginationParams for ListBalanceTransaction<'a> {}
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
impl<'a> RetrieveBalanceTransaction<'a> {
    /// Retrieves the balance transaction with the given ID.
    ///
    /// Note that this endpoint previously used the path `/v1/balance/history/:id`.
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_types::balance_transaction::BalanceTransactionId,
    ) -> stripe::Response<stripe_types::BalanceTransaction> {
        client.get_query(&format!("/balance_transactions/{id}"), self)
    }
}
