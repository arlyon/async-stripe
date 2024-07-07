use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTreasuryTransactionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    financial_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<ListTreasuryTransactionOrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_treasury::TreasuryTransactionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_transitions: Option<ListTreasuryTransactionStatusTransitions>,
}
impl<'a> ListTreasuryTransactionBuilder<'a> {
    fn new(financial_account: &'a str) -> Self {
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryTransactionOrderBy::*;
        match s {
            "created" => Ok(Created),
            "posted_at" => Ok(PostedAt),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryTransactionOrderBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListTreasuryTransactionOrderBy")
        })
    }
}
/// A filter for the `status_transitions.posted_at` timestamp.
/// When using this filter, `status=posted` and `order_by=posted_at` must also be specified.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryTransactionStatusTransitions {
    /// Returns Transactions with `posted_at` within the specified range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<stripe_types::RangeQueryTs>,
}
impl ListTreasuryTransactionStatusTransitions {
    pub fn new() -> Self {
        Self { posted_at: None }
    }
}
impl Default for ListTreasuryTransactionStatusTransitions {
    fn default() -> Self {
        Self::new()
    }
}
/// Retrieves a list of Transaction objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryTransaction<'a> {
    inner: ListTreasuryTransactionBuilder<'a>,
}
impl<'a> ListTreasuryTransaction<'a> {
    /// Construct a new `ListTreasuryTransaction`.
    pub fn new(financial_account: &'a str) -> Self {
        Self { inner: ListTreasuryTransactionBuilder::new(financial_account) }
    }
    /// Only return Transactions that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// The results are in reverse chronological order by `created` or `posted_at`.
    /// The default is `created`.
    pub fn order_by(mut self, order_by: ListTreasuryTransactionOrderBy) -> Self {
        self.inner.order_by = Some(order_by);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Only return Transactions that have the given status: `open`, `posted`, or `void`.
    pub fn status(mut self, status: stripe_treasury::TreasuryTransactionStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
    /// A filter for the `status_transitions.posted_at` timestamp.
    /// When using this filter, `status=posted` and `order_by=posted_at` must also be specified.
    pub fn status_transitions(
        mut self,
        status_transitions: ListTreasuryTransactionStatusTransitions,
    ) -> Self {
        self.inner.status_transitions = Some(status_transitions);
        self
    }
}
impl ListTreasuryTransaction<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_treasury::TreasuryTransaction>>
    {
        stripe_client_core::ListPaginator::new_list("/treasury/transactions", self.inner)
    }
}

impl StripeRequest for ListTreasuryTransaction<'_> {
    type Output = stripe_types::List<stripe_treasury::TreasuryTransaction>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/transactions").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryTransactionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryTransactionBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing Transaction.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryTransaction<'a> {
    inner: RetrieveTreasuryTransactionBuilder<'a>,
    id: &'a stripe_treasury::TreasuryTransactionId,
}
impl<'a> RetrieveTreasuryTransaction<'a> {
    /// Construct a new `RetrieveTreasuryTransaction`.
    pub fn new(id: &'a stripe_treasury::TreasuryTransactionId) -> Self {
        Self { id, inner: RetrieveTreasuryTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTreasuryTransaction<'_> {
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

impl StripeRequest for RetrieveTreasuryTransaction<'_> {
    type Output = stripe_treasury::TreasuryTransaction;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/transactions/{id}"))
            .query(&self.inner)
    }
}
