use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListFinancialConnectionsTransactionBuilder<'a> {
    account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transacted_at: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction_refresh: Option<ListFinancialConnectionsTransactionTransactionRefresh<'a>>,
}
impl<'a> ListFinancialConnectionsTransactionBuilder<'a> {
    fn new(account: &'a str) -> Self {
        Self {
            account,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            transacted_at: None,
            transaction_refresh: None,
        }
    }
}
/// A filter on the list based on the object `transaction_refresh` field.
/// The value can be a dictionary with the following options:.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListFinancialConnectionsTransactionTransactionRefresh<'a> {
    /// Return results where the transactions were created or updated by a refresh that took place after this refresh (non-inclusive).
    pub after: &'a str,
}
impl<'a> ListFinancialConnectionsTransactionTransactionRefresh<'a> {
    pub fn new(after: &'a str) -> Self {
        Self { after }
    }
}
/// Returns a list of Financial Connections `Transaction` objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListFinancialConnectionsTransaction<'a> {
    inner: ListFinancialConnectionsTransactionBuilder<'a>,
}
impl<'a> ListFinancialConnectionsTransaction<'a> {
    /// Construct a new `ListFinancialConnectionsTransaction`.
    pub fn new(account: &'a str) -> Self {
        Self { inner: ListFinancialConnectionsTransactionBuilder::new(account) }
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
    /// A filter on the list based on the object `transacted_at` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with the following options:.
    pub fn transacted_at(mut self, transacted_at: stripe_types::RangeQueryTs) -> Self {
        self.inner.transacted_at = Some(transacted_at);
        self
    }
    /// A filter on the list based on the object `transaction_refresh` field.
    /// The value can be a dictionary with the following options:.
    pub fn transaction_refresh(
        mut self,
        transaction_refresh: ListFinancialConnectionsTransactionTransactionRefresh<'a>,
    ) -> Self {
        self.inner.transaction_refresh = Some(transaction_refresh);
        self
    }
}
impl ListFinancialConnectionsTransaction<'_> {
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
        stripe_types::List<stripe_misc::FinancialConnectionsTransaction>,
    > {
        stripe_client_core::ListPaginator::new_list(
            "/financial_connections/transactions",
            self.inner,
        )
    }
}

impl StripeRequest for ListFinancialConnectionsTransaction<'_> {
    type Output = stripe_types::List<stripe_misc::FinancialConnectionsTransaction>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/financial_connections/transactions")
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveFinancialConnectionsTransactionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFinancialConnectionsTransactionBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a Financial Connections `Transaction`
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFinancialConnectionsTransaction<'a> {
    inner: RetrieveFinancialConnectionsTransactionBuilder<'a>,
    transaction: &'a stripe_misc::FinancialConnectionsTransactionId,
}
impl<'a> RetrieveFinancialConnectionsTransaction<'a> {
    /// Construct a new `RetrieveFinancialConnectionsTransaction`.
    pub fn new(transaction: &'a stripe_misc::FinancialConnectionsTransactionId) -> Self {
        Self { transaction, inner: RetrieveFinancialConnectionsTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveFinancialConnectionsTransaction<'_> {
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

impl StripeRequest for RetrieveFinancialConnectionsTransaction<'_> {
    type Output = stripe_misc::FinancialConnectionsTransaction;

    fn build(&self) -> RequestBuilder {
        let transaction = self.transaction;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/financial_connections/transactions/{transaction}"),
        )
        .query(&self.inner)
    }
}
