use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListFinancialConnectionsTransactionBuilder {
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transacted_at: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction_refresh: Option<ListFinancialConnectionsTransactionTransactionRefresh>,
}
impl ListFinancialConnectionsTransactionBuilder {
    fn new(account: impl Into<String>) -> Self {
        Self {
            account: account.into(),
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ListFinancialConnectionsTransactionTransactionRefresh {
    /// Return results where the transactions were created or updated by a refresh that took place after this refresh (non-inclusive).
    pub after: String,
}
impl ListFinancialConnectionsTransactionTransactionRefresh {
    pub fn new(after: impl Into<String>) -> Self {
        Self { after: after.into() }
    }
}
/// Returns a list of Financial Connections `Transaction` objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListFinancialConnectionsTransaction {
    inner: ListFinancialConnectionsTransactionBuilder,
}
impl ListFinancialConnectionsTransaction {
    /// Construct a new `ListFinancialConnectionsTransaction`.
    pub fn new(account: impl Into<String>) -> Self {
        Self { inner: ListFinancialConnectionsTransactionBuilder::new(account.into()) }
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// A filter on the list based on the object `transacted_at` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with the following options:.
    pub fn transacted_at(mut self, transacted_at: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.transacted_at = Some(transacted_at.into());
        self
    }
    /// A filter on the list based on the object `transaction_refresh` field.
    /// The value can be a dictionary with the following options:.
    pub fn transaction_refresh(
        mut self,
        transaction_refresh: impl Into<ListFinancialConnectionsTransactionTransactionRefresh>,
    ) -> Self {
        self.inner.transaction_refresh = Some(transaction_refresh.into());
        self
    }
}
impl ListFinancialConnectionsTransaction {
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
            &self.inner,
        )
    }
}

impl StripeRequest for ListFinancialConnectionsTransaction {
    type Output = stripe_types::List<stripe_misc::FinancialConnectionsTransaction>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/financial_connections/transactions")
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveFinancialConnectionsTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveFinancialConnectionsTransactionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a Financial Connections `Transaction`
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFinancialConnectionsTransaction {
    inner: RetrieveFinancialConnectionsTransactionBuilder,
    transaction: stripe_misc::FinancialConnectionsTransactionId,
}
impl RetrieveFinancialConnectionsTransaction {
    /// Construct a new `RetrieveFinancialConnectionsTransaction`.
    pub fn new(transaction: impl Into<stripe_misc::FinancialConnectionsTransactionId>) -> Self {
        Self {
            transaction: transaction.into(),
            inner: RetrieveFinancialConnectionsTransactionBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveFinancialConnectionsTransaction {
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

impl StripeRequest for RetrieveFinancialConnectionsTransaction {
    type Output = stripe_misc::FinancialConnectionsTransaction;

    fn build(&self) -> RequestBuilder {
        let transaction = &self.transaction;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/financial_connections/transactions/{transaction}"),
        )
        .query(&self.inner)
    }
}
