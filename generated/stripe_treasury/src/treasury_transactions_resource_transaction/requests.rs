#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryTransactionsResourceTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryTransactionsResourceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryTransactionsResourceTransaction<'a> {
    /// Retrieves the details of an existing Transaction.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_transactions_resource_transaction::TreasuryTransactionId) -> stripe::Response<stripe_treasury::TreasuryTransactionsResourceTransaction> {
        client.get_query(&format!("/treasury/transactions/{id}", id = id), self)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryTransactionsResourceTransaction<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
    /// The results are in reverse chronological order by `created` or `posted_at`.
    ///
    /// The default is `created`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<ListTreasuryTransactionsResourceTransactionOrderBy>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return Transactions that have the given status: `open`, `posted`, or `void`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTreasuryTransactionsResourceTransactionStatus>,
    /// A filter for the `status_transitions.posted_at` timestamp.
    ///
    /// When using this filter, `status=posted` and `order_by=posted_at` must also be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<ListTreasuryTransactionsResourceTransactionStatusTransitions>,
}
impl<'a> ListTreasuryTransactionsResourceTransaction<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            financial_account,
            limit: Default::default(),
            order_by: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
            status_transitions: Default::default(),
        }
    }
}
/// The results are in reverse chronological order by `created` or `posted_at`.
///
/// The default is `created`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryTransactionsResourceTransactionOrderBy {
    Created,
    PostedAt,
}

impl ListTreasuryTransactionsResourceTransactionOrderBy {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryTransactionsResourceTransactionOrderBy::*;
        match self {
            Created => "created",
            PostedAt => "posted_at",
        }
    }
}

impl std::str::FromStr for ListTreasuryTransactionsResourceTransactionOrderBy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryTransactionsResourceTransactionOrderBy::*;
        match s {
            "created" => Ok(Created),
            "posted_at" => Ok(PostedAt),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryTransactionsResourceTransactionOrderBy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryTransactionsResourceTransactionOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryTransactionsResourceTransactionOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryTransactionsResourceTransactionOrderBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Only return Transactions that have the given status: `open`, `posted`, or `void`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryTransactionsResourceTransactionStatus {
    Open,
    Posted,
    Void,
}

impl ListTreasuryTransactionsResourceTransactionStatus {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryTransactionsResourceTransactionStatus::*;
        match self {
            Open => "open",
            Posted => "posted",
            Void => "void",
        }
    }
}

impl std::str::FromStr for ListTreasuryTransactionsResourceTransactionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryTransactionsResourceTransactionStatus::*;
        match s {
            "open" => Ok(Open),
            "posted" => Ok(Posted),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryTransactionsResourceTransactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryTransactionsResourceTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryTransactionsResourceTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryTransactionsResourceTransactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// A filter for the `status_transitions.posted_at` timestamp.
///
/// When using this filter, `status=posted` and `order_by=posted_at` must also be specified.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTreasuryTransactionsResourceTransactionStatusTransitions {
    /// Returns Transactions with `posted_at` within the specified range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<stripe_types::RangeQueryTs>,
}
impl ListTreasuryTransactionsResourceTransactionStatusTransitions {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTreasuryTransactionsResourceTransaction<'a> {
    /// Retrieves a list of Transaction objects.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryTransactionsResourceTransaction>> {
        client.get_query("/treasury/transactions", self)
    }
}
