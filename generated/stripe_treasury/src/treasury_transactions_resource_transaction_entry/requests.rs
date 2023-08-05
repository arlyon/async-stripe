#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryTransactionsResourceTransactionEntry<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryTransactionsResourceTransactionEntry<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryTransactionsResourceTransactionEntry<'a> {
    /// Retrieves a TransactionEntry object.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_transactions_resource_transaction_entry::TreasuryTransactionEntryId) -> stripe::Response<stripe_treasury::TreasuryTransactionsResourceTransactionEntry> {
        client.get_query(&format!("/treasury/transaction_entries/{id}", id = id), self)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryTransactionsResourceTransactionEntry<'a> {
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
    pub order_by: Option<ListTreasuryTransactionsResourceTransactionEntryOrderBy>,
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
impl<'a> ListTreasuryTransactionsResourceTransactionEntry<'a> {
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryTransactionsResourceTransactionEntryOrderBy {
    Created,
    EffectiveAt,
}

impl ListTreasuryTransactionsResourceTransactionEntryOrderBy {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryTransactionsResourceTransactionEntryOrderBy::*;
        match self {
            Created => "created",
            EffectiveAt => "effective_at",
        }
    }
}

impl std::str::FromStr for ListTreasuryTransactionsResourceTransactionEntryOrderBy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryTransactionsResourceTransactionEntryOrderBy::*;
        match s {
            "created" => Ok(Created),
            "effective_at" => Ok(EffectiveAt),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryTransactionsResourceTransactionEntryOrderBy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryTransactionsResourceTransactionEntryOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryTransactionsResourceTransactionEntryOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryTransactionsResourceTransactionEntryOrderBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> stripe::PaginationParams for ListTreasuryTransactionsResourceTransactionEntry<'a> {}
impl<'a> ListTreasuryTransactionsResourceTransactionEntry<'a> {
    /// Retrieves a list of TransactionEntry objects.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryTransactionsResourceTransactionEntry>> {
        client.get_query("/treasury/transaction_entries", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_treasury::TreasuryTransactionsResourceTransactionEntry> {
        stripe::ListPaginator::from_params("/treasury/transaction_entries", self)
    }
}
