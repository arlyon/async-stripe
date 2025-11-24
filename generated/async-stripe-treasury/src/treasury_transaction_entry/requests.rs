use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListTreasuryTransactionEntryBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<ListTreasuryTransactionEntryOrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction: Option<String>,
}
impl ListTreasuryTransactionEntryBuilder {
    fn new(financial_account: impl Into<String>) -> Self {
        Self {
            created: None,
            effective_at: None,
            ending_before: None,
            expand: None,
            financial_account: financial_account.into(),
            limit: None,
            order_by: None,
            starting_after: None,
            transaction: None,
        }
    }
}
/// The results are in reverse chronological order by `created` or `effective_at`.
/// The default is `created`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListTreasuryTransactionEntryOrderBy {
    Created,
    EffectiveAt,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListTreasuryTransactionEntryOrderBy {
    pub fn as_str(&self) -> &str {
        use ListTreasuryTransactionEntryOrderBy::*;
        match self {
            Created => "created",
            EffectiveAt => "effective_at",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListTreasuryTransactionEntryOrderBy {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryTransactionEntryOrderBy::*;
        match s {
            "created" => Ok(Created),
            "effective_at" => Ok(EffectiveAt),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ListTreasuryTransactionEntryOrderBy"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ListTreasuryTransactionEntryOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryTransactionEntryOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryTransactionEntryOrderBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryTransactionEntryOrderBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Retrieves a list of TransactionEntry objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryTransactionEntry {
    inner: ListTreasuryTransactionEntryBuilder,
}
impl ListTreasuryTransactionEntry {
    /// Construct a new `ListTreasuryTransactionEntry`.
    pub fn new(financial_account: impl Into<String>) -> Self {
        Self { inner: ListTreasuryTransactionEntryBuilder::new(financial_account.into()) }
    }
    /// Only return TransactionEntries that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    pub fn effective_at(mut self, effective_at: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.effective_at = Some(effective_at.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// The results are in reverse chronological order by `created` or `effective_at`.
    /// The default is `created`.
    pub fn order_by(mut self, order_by: impl Into<ListTreasuryTransactionEntryOrderBy>) -> Self {
        self.inner.order_by = Some(order_by.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return TransactionEntries associated with this Transaction.
    pub fn transaction(mut self, transaction: impl Into<String>) -> Self {
        self.inner.transaction = Some(transaction.into());
        self
    }
}
impl ListTreasuryTransactionEntry {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_treasury::TreasuryTransactionEntry>,
    > {
        stripe_client_core::ListPaginator::new_list("/treasury/transaction_entries", &self.inner)
    }
}

impl StripeRequest for ListTreasuryTransactionEntry {
    type Output = stripe_types::List<stripe_treasury::TreasuryTransactionEntry>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/transaction_entries").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryTransactionEntryBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTreasuryTransactionEntryBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a TransactionEntry object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryTransactionEntry {
    inner: RetrieveTreasuryTransactionEntryBuilder,
    id: stripe_treasury::TreasuryTransactionEntryId,
}
impl RetrieveTreasuryTransactionEntry {
    /// Construct a new `RetrieveTreasuryTransactionEntry`.
    pub fn new(id: impl Into<stripe_treasury::TreasuryTransactionEntryId>) -> Self {
        Self { id: id.into(), inner: RetrieveTreasuryTransactionEntryBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTreasuryTransactionEntry {
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

impl StripeRequest for RetrieveTreasuryTransactionEntry {
    type Output = stripe_treasury::TreasuryTransactionEntry;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/transaction_entries/{id}"))
            .query(&self.inner)
    }
}
