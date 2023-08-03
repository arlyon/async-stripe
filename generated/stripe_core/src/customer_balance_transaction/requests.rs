
/// Retrieves a specific customer balance transaction that updated the customer’s [balances](https://stripe.com/docs/billing/customer/balance).
pub fn retrieve(client: &stripe::Client, customer: &stripe_types::customer::CustomerId, transaction: &str, params: RetrieveCustomerBalanceTransaction) -> stripe::Response<stripe_types::CustomerBalanceTransaction> {
    client.get_query(&format!("/customers/{customer}/balance_transactions/{transaction}", customer = customer, transaction = transaction), params)
}
/// Returns a list of transactions that updated the customer’s [balances](https://stripe.com/docs/billing/customer/balance).
pub fn list(client: &stripe::Client, customer: &stripe_types::customer::CustomerId, params: ListCustomerBalanceTransaction) -> stripe::Response<stripe_types::List<stripe_types::CustomerBalanceTransaction>> {
    client.get_query(&format!("/customers/{customer}/balance_transactions", customer = customer), params)
}
/// Creates an immutable transaction that updates the customer’s credit [balance](https://stripe.com/docs/billing/customer/balance).
pub fn create(client: &stripe::Client, customer: &stripe_types::customer::CustomerId, params: CreateCustomerBalanceTransaction) -> stripe::Response<stripe_types::CustomerBalanceTransaction> {
    client.send_form(&format!("/customers/{customer}/balance_transactions", customer = customer), params, http_types::Method::Post)
}
/// Most credit balance transaction fields are immutable, but you may update its `description` and `metadata`.
pub fn update(client: &stripe::Client, customer: &stripe_types::customer::CustomerId, transaction: &str, params: UpdateCustomerBalanceTransaction) -> stripe::Response<stripe_types::CustomerBalanceTransaction> {
    client.send_form(&format!("/customers/{customer}/balance_transactions/{transaction}", customer = customer, transaction = transaction), params, http_types::Method::Post)
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCustomerBalanceTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCustomerBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListCustomerBalanceTransaction<'a> {
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
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListCustomerBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerBalanceTransaction<'a> {
    /// The integer amount in **cents (or local equivalent)** to apply to the customer's credit balance.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Specifies the [`invoice_credit_balance`](https://stripe.com/docs/api/customers/object#customer_object-invoice_credit_balance) that this transaction will apply to.
    /// If the customer's `currency` is not set, it will be updated to this value.
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateCustomerBalanceTransaction<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, description: Default::default(), expand: Default::default(), metadata: Default::default() }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerBalanceTransaction<'a> {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateCustomerBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
