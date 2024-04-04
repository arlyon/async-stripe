#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListCustomerCustomerCashBalanceTransaction<'a> {
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
}
impl<'a> ListCustomerCustomerCashBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListCustomerCustomerCashBalanceTransaction<'a> {
    /// Returns a list of transactions that modified the customer’s [cash balance](https://stripe.com/docs/payments/customer-balance).
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::Response<stripe_types::List<stripe_shared::CustomerCashBalanceTransaction>> {
        client.get_query(&format!("/customers/{customer}/cash_balance_transactions"), self)
    }
    pub fn paginate(
        self,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_shared::CustomerCashBalanceTransaction>>
    {
        stripe::ListPaginator::from_list_params(
            &format!("/customers/{customer}/cash_balance_transactions"),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCustomerCashBalanceTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCustomerCashBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveCustomerCashBalanceTransaction<'a> {
    /// Retrieves a specific cash balance transaction, which updated the customer’s [cash balance](https://stripe.com/docs/payments/customer-balance).
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
        transaction: &str,
    ) -> stripe::Response<stripe_shared::CustomerCashBalanceTransaction> {
        client.get_query(
            &format!("/customers/{customer}/cash_balance_transactions/{transaction}"),
            self,
        )
    }
}
