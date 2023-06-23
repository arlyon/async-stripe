use stripe::{Client, Response};

impl stripe_treasury::treasury::transaction_entry::TransactionEntry {
    /// Retrieves a TransactionEntry object.
    pub fn retrieve(
        client: &Client,
        id: &str,
        params: RetrieveTransactionEntry,
    ) -> Response<stripe_treasury::treasury::transaction_entry::TransactionEntry> {
        client.get_query(&format!("/treasury/transaction_entries/{id}", id = id), params)
    }
    /// Retrieves a list of TransactionEntry objects.
    pub fn list(
        client: &Client,
        params: ListTransactionEntry,
    ) -> Response<stripe_types::List<stripe_treasury::treasury::transaction_entry::TransactionEntry>>
    {
        client.get_query("/treasury/transaction_entries", params)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTransactionEntry<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTransactionEntry<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTransactionEntry<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Returns objects associated with this FinancialAccount.
    pub financial_account: &'a str,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The results are in reverse chronological order by `created` or `effective_at`.
    ///
    /// The default is `created`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<ListTransactionEntryOrderBy>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return TransactionEntries associated with this Transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<&'a str>,
}
impl<'a> ListTransactionEntry<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            created: Default::default(),
            effective_at: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            financial_account,
            limit: Default::default(),
            order_by: Default::default(),
            starting_after: Default::default(),
            transaction: Default::default(),
        }
    }
}
/// The results are in reverse chronological order by `created` or `effective_at`.
///
/// The default is `created`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListTransactionEntryOrderBy {
    Created,
    EffectiveAt,
}

impl ListTransactionEntryOrderBy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::EffectiveAt => "effective_at",
        }
    }
}

impl AsRef<str> for ListTransactionEntryOrderBy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTransactionEntryOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
