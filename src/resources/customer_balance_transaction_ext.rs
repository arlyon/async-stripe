use serde::Serialize;

use crate::client::{Client, Response};
use crate::ids::{CustomerBalanceTransactionId, CustomerId};
use crate::params::{Expand, List, Metadata, Paginable};
use crate::resources::{Currency, Customer, CustomerBalanceTransaction};

/// The parameters for `CustomerBalanceTransaction::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCustomerBalanceTransactions<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects,
    /// starting with `obj_bar`, your subsequent call can include
    /// `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<CustomerBalanceTransactionId>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects,
    /// ending with `obj_foo`, your subsequent call can include
    /// `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<CustomerBalanceTransactionId>,
}

impl Paginable for ListCustomerBalanceTransactions<'_> {
    type O = CustomerBalanceTransaction;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id);
    }
}

/// The parameters that can be used when creating or updating a [`CustomerBalanceTransaction`].
#[derive(Clone, Debug, Serialize)]
pub struct CreateCustomerBalanceTransaction<'a> {
    /// The integer amount in cents to apply to the customer’s credit balance.
    pub amount: i64,
    /// Three-letter ISO currency code, in lowercase.
    ///
    /// Must be a supported currency. Specifies the invoice_credit_balance that this
    /// transaction will apply to. If the customer’s currency is not set, it will be
    /// updated to this value.
    pub currency: Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a
    /// structured format. Individual keys can be unset by posting an empty value to
    /// them. All keys can be unset by posting an empty value to metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateCustomerBalanceTransaction<'_> {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency, description: Default::default(), metadata: Default::default() }
    }
}

/// The parameters that can be used when creating or updating a [`CustomerBalanceTransaction`].
///
/// Only the description and metadata fields can be updated.
#[derive(Clone, Debug, Default, Serialize)]
pub struct UpdateCustomerBalanceTransaction<'a> {
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a
    /// structured format. Individual keys can be unset by posting an empty value to
    /// them. All keys can be unset by posting an empty value to metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl Customer {
    /// List all of a customer's balance transactions.
    pub fn list_balance_transactions(
        client: &Client,
        customer_id: &CustomerId,
        params: ListCustomerBalanceTransactions<'_>,
    ) -> Response<List<CustomerBalanceTransaction>> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.get_query(&format!("/customers/{}/balance_transactions", customer_id), &params)
    }

    /// Create a new customer balance transaction.
    pub fn create_balance_transaction(
        client: &Client,
        customer_id: &CustomerId,
        params: CreateCustomerBalanceTransaction<'_>,
    ) -> Response<CustomerBalanceTransaction> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/customers/{}/balance_transactions", customer_id), &params)
    }

    /// Retrieve a customer balance transaction.
    pub fn retrieve_balance_transaction(
        client: &Client,
        customer_id: &CustomerId,
        id: &CustomerBalanceTransactionId,
        expand: &[&str],
    ) -> Response<CustomerBalanceTransaction> {
        client.get_query(
            &format!("/customers/{}/balance_transactions/{}", customer_id, id),
            Expand { expand },
        )
    }

    /// Update a customer balance transaction.
    ///
    /// Only the description and metadata fields can be updated.
    pub fn update_balance_transaction(
        client: &Client,
        customer_id: &CustomerId,
        id: &CustomerBalanceTransactionId,
        params: UpdateCustomerBalanceTransaction<'_>,
    ) -> Response<CustomerBalanceTransaction> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client
            .post_form(&format!("/customers/{}/balance_transactions/{}", customer_id, id), &params)
    }
}
