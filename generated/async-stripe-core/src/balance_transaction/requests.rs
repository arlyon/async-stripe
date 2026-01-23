use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListBalanceTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<String>,
}
impl ListBalanceTransactionBuilder {
    fn new() -> Self {
        Self {
            created: None,
            currency: None,
            ending_before: None,
            expand: None,
            limit: None,
            payout: None,
            source: None,
            starting_after: None,
            type_: None,
        }
    }
}
/// Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth).
/// The transactions are returned in sorted order, with the most recent transactions appearing first.
///
/// Note that this endpoint was previously called “Balance history” and used the path `/v1/balance/history`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListBalanceTransaction {
    inner: ListBalanceTransactionBuilder,
}
impl ListBalanceTransaction {
    /// Construct a new `ListBalanceTransaction`.
    pub fn new() -> Self {
        Self { inner: ListBalanceTransactionBuilder::new() }
    }
    /// Only return transactions that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return transactions in a certain currency.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// For automatic Stripe payouts only, only returns transactions that were paid out on the specified payout ID.
    pub fn payout(mut self, payout: impl Into<String>) -> Self {
        self.inner.payout = Some(payout.into());
        self
    }
    /// Only returns transactions associated with the given object.
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.inner.source = Some(source.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only returns transactions of the given type.
    /// One of: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `climate_order_purchase`, `climate_order_refund`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `obligation_outbound`, `obligation_reversal_inbound`, `payment`, `payment_failure_refund`, `payment_network_reserve_hold`, `payment_network_reserve_release`, `payment_refund`, `payment_reversal`, `payment_unreconciled`, `payout`, `payout_cancel`, `payout_failure`, `payout_minimum_balance_hold`, `payout_minimum_balance_release`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `reserve_hold`, `reserve_release`, `stripe_fee`, `stripe_fx_fee`, `stripe_balance_payment_debit`, `stripe_balance_payment_debit_reversal`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl Default for ListBalanceTransaction {
    fn default() -> Self {
        Self::new()
    }
}
impl ListBalanceTransaction {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::BalanceTransaction>>
    {
        stripe_client_core::ListPaginator::new_list("/balance_transactions", &self.inner)
    }
}

impl StripeRequest for ListBalanceTransaction {
    type Output = stripe_types::List<stripe_shared::BalanceTransaction>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/balance_transactions").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveBalanceTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveBalanceTransactionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the balance transaction with the given ID.
///
/// Note that this endpoint previously used the path `/v1/balance/history/:id`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveBalanceTransaction {
    inner: RetrieveBalanceTransactionBuilder,
    id: stripe_shared::BalanceTransactionId,
}
impl RetrieveBalanceTransaction {
    /// Construct a new `RetrieveBalanceTransaction`.
    pub fn new(id: impl Into<stripe_shared::BalanceTransactionId>) -> Self {
        Self { id: id.into(), inner: RetrieveBalanceTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveBalanceTransaction {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveBalanceTransaction {
    type Output = stripe_shared::BalanceTransaction;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/balance_transactions/{id}"))
            .query(&self.inner)
    }
}
