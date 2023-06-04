use crate::{Client, Response};

impl crate::issuing::transaction::Transaction {
    /// Returns a list of Issuing `Transaction` objects.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn list(
        client: &Client,
        params: ListTransaction,
    ) -> Response<crate::List<crate::issuing::transaction::Transaction>> {
        client.get_query("/issuing/transactions", params)
    }
    /// Retrieves an Issuing `Transaction` object.
    pub fn retrieve(
        client: &Client,
        transaction: &str,
        params: RetrieveTransaction,
    ) -> Response<crate::issuing::transaction::Transaction> {
        client.get_query(
            &format!("/issuing/transactions/{transaction}", transaction = transaction),
            params,
        )
    }
    /// Updates the specified Issuing `Transaction` object by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(
        client: &Client,
        transaction: &str,
        params: UpdateTransaction,
    ) -> Response<crate::issuing::transaction::Transaction> {
        client.send_form(
            &format!("/issuing/transactions/{transaction}", transaction = transaction),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListTransaction<'a> {
    /// Only return transactions that belong to the given card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<&'a str>,
    /// Only return transactions that belong to the given cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<&'a str>,
    /// Only return transactions that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::RangeQueryTs>,
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
    /// Only return transactions that have the given type.
    ///
    /// One of `capture` or `refund`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ListTransactionType>,
}
impl<'a> ListTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return transactions that have the given type.
///
/// One of `capture` or `refund`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListTransactionType {
    Capture,
    Refund,
}

impl ListTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Refund => "refund",
        }
    }
}

impl AsRef<str> for ListTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
}
impl<'a> UpdateTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
