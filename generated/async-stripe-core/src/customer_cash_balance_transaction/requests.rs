use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListCustomerCustomerCashBalanceTransactionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListCustomerCustomerCashBalanceTransactionBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of transactions that modified the customer’s [cash balance](https://stripe.com/docs/payments/customer-balance).
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCustomerCustomerCashBalanceTransaction<'a> {
    inner: ListCustomerCustomerCashBalanceTransactionBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> ListCustomerCustomerCashBalanceTransaction<'a> {
    /// Construct a new `ListCustomerCustomerCashBalanceTransaction`.
    pub fn new(customer: &'a stripe_shared::CustomerId) -> Self {
        Self { customer, inner: ListCustomerCustomerCashBalanceTransactionBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl ListCustomerCustomerCashBalanceTransaction<'_> {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_shared::CustomerCashBalanceTransaction>,
    > {
        let customer = self.customer;

        stripe_client_core::ListPaginator::new_list(
            format!("/customers/{customer}/cash_balance_transactions"),
            self.inner,
        )
    }
}

impl StripeRequest for ListCustomerCustomerCashBalanceTransaction<'_> {
    type Output = stripe_types::List<stripe_shared::CustomerCashBalanceTransaction>;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/customers/{customer}/cash_balance_transactions"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveCustomerCashBalanceTransactionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCustomerCashBalanceTransactionBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a specific cash balance transaction, which updated the customer’s [cash balance](https://stripe.com/docs/payments/customer-balance).
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCustomerCashBalanceTransaction<'a> {
    inner: RetrieveCustomerCashBalanceTransactionBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
    transaction: &'a str,
}
impl<'a> RetrieveCustomerCashBalanceTransaction<'a> {
    /// Construct a new `RetrieveCustomerCashBalanceTransaction`.
    pub fn new(customer: &'a stripe_shared::CustomerId, transaction: &'a str) -> Self {
        Self { customer, transaction, inner: RetrieveCustomerCashBalanceTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveCustomerCashBalanceTransaction<'_> {
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

impl StripeRequest for RetrieveCustomerCashBalanceTransaction<'_> {
    type Output = stripe_shared::CustomerCashBalanceTransaction;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        let transaction = self.transaction;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/customers/{customer}/cash_balance_transactions/{transaction}"),
        )
        .query(&self.inner)
    }
}
