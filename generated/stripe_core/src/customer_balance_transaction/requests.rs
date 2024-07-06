use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListCustomerCustomerBalanceTransactionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListCustomerCustomerBalanceTransactionBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of transactions that updated the customer’s [balances](https://stripe.com/docs/billing/customer/balance).
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCustomerCustomerBalanceTransaction<'a> {
    inner: ListCustomerCustomerBalanceTransactionBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> ListCustomerCustomerBalanceTransaction<'a> {
    /// Construct a new `ListCustomerCustomerBalanceTransaction`.
    pub fn new(customer: &'a stripe_shared::CustomerId) -> Self {
        Self { customer, inner: ListCustomerCustomerBalanceTransactionBuilder::new() }
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
impl ListCustomerCustomerBalanceTransaction<'_> {
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
        stripe_types::List<stripe_shared::CustomerBalanceTransaction>,
    > {
        let customer = self.customer;

        stripe_client_core::ListPaginator::new_list(
            format!("/customers/{customer}/balance_transactions"),
            self.inner,
        )
    }
}

impl StripeRequest for ListCustomerCustomerBalanceTransaction<'_> {
    type Output = stripe_types::List<stripe_shared::CustomerBalanceTransaction>;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/customers/{customer}/balance_transactions"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveCustomerBalanceTransactionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCustomerBalanceTransactionBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a specific customer balance transaction that updated the customer’s [balances](https://stripe.com/docs/billing/customer/balance).
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCustomerBalanceTransaction<'a> {
    inner: RetrieveCustomerBalanceTransactionBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
    transaction: &'a str,
}
impl<'a> RetrieveCustomerBalanceTransaction<'a> {
    /// Construct a new `RetrieveCustomerBalanceTransaction`.
    pub fn new(customer: &'a stripe_shared::CustomerId, transaction: &'a str) -> Self {
        Self { customer, transaction, inner: RetrieveCustomerBalanceTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveCustomerBalanceTransaction<'_> {
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

impl StripeRequest for RetrieveCustomerBalanceTransaction<'_> {
    type Output = stripe_shared::CustomerBalanceTransaction;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        let transaction = self.transaction;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/customers/{customer}/balance_transactions/{transaction}"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateCustomerCustomerBalanceTransactionBuilder<'a> {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateCustomerCustomerBalanceTransactionBuilder<'a> {
    fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, description: None, expand: None, metadata: None }
    }
}
/// Creates an immutable transaction that updates the customer’s credit [balance](https://stripe.com/docs/billing/customer/balance).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerCustomerBalanceTransaction<'a> {
    inner: CreateCustomerCustomerBalanceTransactionBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> CreateCustomerCustomerBalanceTransaction<'a> {
    /// Construct a new `CreateCustomerCustomerBalanceTransaction`.
    pub fn new(
        customer: &'a stripe_shared::CustomerId,
        amount: i64,
        currency: stripe_types::Currency,
    ) -> Self {
        Self {
            customer,
            inner: CreateCustomerCustomerBalanceTransactionBuilder::new(amount, currency),
        }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl CreateCustomerCustomerBalanceTransaction<'_> {
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

impl StripeRequest for CreateCustomerCustomerBalanceTransaction<'_> {
    type Output = stripe_shared::CustomerBalanceTransaction;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/customers/{customer}/balance_transactions"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateCustomerBalanceTransactionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateCustomerBalanceTransactionBuilder<'a> {
    fn new() -> Self {
        Self { description: None, expand: None, metadata: None }
    }
}
/// Most credit balance transaction fields are immutable, but you may update its `description` and `metadata`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerBalanceTransaction<'a> {
    inner: UpdateCustomerBalanceTransactionBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
    transaction: &'a str,
}
impl<'a> UpdateCustomerBalanceTransaction<'a> {
    /// Construct a new `UpdateCustomerBalanceTransaction`.
    pub fn new(customer: &'a stripe_shared::CustomerId, transaction: &'a str) -> Self {
        Self { customer, transaction, inner: UpdateCustomerBalanceTransactionBuilder::new() }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl UpdateCustomerBalanceTransaction<'_> {
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

impl StripeRequest for UpdateCustomerBalanceTransaction<'_> {
    type Output = stripe_shared::CustomerBalanceTransaction;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        let transaction = self.transaction;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/customers/{customer}/balance_transactions/{transaction}"),
        )
        .form(&self.inner)
    }
}
