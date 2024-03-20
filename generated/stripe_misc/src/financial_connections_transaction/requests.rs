#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListFinancialConnectionsTransaction<'a> {
    /// The ID of the Stripe account whose transactions will be retrieved.
    pub account: &'a str,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// A filter on the list based on the object `transacted_at` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with the following options:.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transacted_at: Option<stripe_types::RangeQueryTs>,
    /// A filter on the list based on the object `transaction_refresh` field.
    /// The value can be a dictionary with the following options:.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_refresh: Option<ListFinancialConnectionsTransactionTransactionRefresh<'a>>,
}
impl<'a> ListFinancialConnectionsTransaction<'a> {
    pub fn new(account: &'a str) -> Self {
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
impl<'a> ListFinancialConnectionsTransaction<'a> {
    /// Returns a list of Financial Connections `Transaction` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::FinancialConnectionsTransaction>> {
        client.get_query("/financial_connections/transactions", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::FinancialConnectionsTransaction>>
    {
        stripe::ListPaginator::from_list_params("/financial_connections/transactions", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFinancialConnectionsTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFinancialConnectionsTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveFinancialConnectionsTransaction<'a> {
    /// Retrieves the details of a Financial Connections `Transaction`
    pub fn send(
        &self,
        client: &stripe::Client,
        transaction: &stripe_misc::FinancialConnectionsTransactionId,
    ) -> stripe::Response<stripe_misc::FinancialConnectionsTransaction> {
        client.get_query(&format!("/financial_connections/transactions/{transaction}"), self)
    }
}
