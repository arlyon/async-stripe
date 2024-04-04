#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryTransaction<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Returns objects associated with this FinancialAccount.
    pub financial_account: &'a str,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The results are in reverse chronological order by `created` or `posted_at`.
    /// The default is `created`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<ListTreasuryTransactionOrderBy>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return Transactions that have the given status: `open`, `posted`, or `void`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_treasury::TreasuryTransactionStatus>,
    /// A filter for the `status_transitions.posted_at` timestamp.
    /// When using this filter, `status=posted` and `order_by=posted_at` must also be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<ListTreasuryTransactionStatusTransitions>,
}
impl<'a> ListTreasuryTransaction<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            created: None,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            order_by: None,
            starting_after: None,
            status: None,
            status_transitions: None,
        }
    }
}
/// The results are in reverse chronological order by `created` or `posted_at`.
/// The default is `created`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryTransactionOrderBy {
    Created,
    PostedAt,
}
impl ListTreasuryTransactionOrderBy {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryTransactionOrderBy::*;
        match self {
            Created => "created",
            PostedAt => "posted_at",
        }
    }
}

impl std::str::FromStr for ListTreasuryTransactionOrderBy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryTransactionOrderBy::*;
        match s {
            "created" => Ok(Created),
            "posted_at" => Ok(PostedAt),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ListTreasuryTransactionOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryTransactionOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryTransactionOrderBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// A filter for the `status_transitions.posted_at` timestamp.
/// When using this filter, `status=posted` and `order_by=posted_at` must also be specified.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTreasuryTransactionStatusTransitions {
    /// Returns Transactions with `posted_at` within the specified range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<stripe_types::RangeQueryTs>,
}
impl ListTreasuryTransactionStatusTransitions {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTreasuryTransaction<'a> {
    /// Retrieves a list of Transaction objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryTransaction>> {
        client.get_query("/treasury/transactions", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_treasury::TreasuryTransaction>> {
        stripe::ListPaginator::from_list_params("/treasury/transactions", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryTransaction<'a> {
    /// Retrieves the details of an existing Transaction.
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_treasury::TreasuryTransactionId,
    ) -> stripe::Response<stripe_treasury::TreasuryTransaction> {
        client.get_query(&format!("/treasury/transactions/{id}"), self)
    }
}
