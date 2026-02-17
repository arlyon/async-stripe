use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListTreasuryCreditReversalBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    received_credit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_treasury::TreasuryCreditReversalStatus>,
}
impl ListTreasuryCreditReversalBuilder {
    fn new(financial_account: impl Into<String>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            financial_account: financial_account.into(),
            limit: None,
            received_credit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of CreditReversals.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryCreditReversal {
    inner: ListTreasuryCreditReversalBuilder,
}
impl ListTreasuryCreditReversal {
    /// Construct a new `ListTreasuryCreditReversal`.
    pub fn new(financial_account: impl Into<String>) -> Self {
        Self { inner: ListTreasuryCreditReversalBuilder::new(financial_account.into()) }
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
    /// Only return CreditReversals for the ReceivedCredit ID.
    pub fn received_credit(mut self, received_credit: impl Into<String>) -> Self {
        self.inner.received_credit = Some(received_credit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return CreditReversals for a given status.
    pub fn status(
        mut self,
        status: impl Into<stripe_treasury::TreasuryCreditReversalStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl ListTreasuryCreditReversal {
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
        stripe_types::List<stripe_treasury::TreasuryCreditReversal>,
    > {
        stripe_client_core::ListPaginator::new_list("/treasury/credit_reversals", &self.inner)
    }
}

impl StripeRequest for ListTreasuryCreditReversal {
    type Output = stripe_types::List<stripe_treasury::TreasuryCreditReversal>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/credit_reversals").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveTreasuryCreditReversalBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTreasuryCreditReversalBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing CreditReversal by passing the unique CreditReversal ID from either the CreditReversal creation request or CreditReversal list.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryCreditReversal {
    inner: RetrieveTreasuryCreditReversalBuilder,
    credit_reversal: stripe_treasury::TreasuryCreditReversalId,
}
impl RetrieveTreasuryCreditReversal {
    /// Construct a new `RetrieveTreasuryCreditReversal`.
    pub fn new(credit_reversal: impl Into<stripe_treasury::TreasuryCreditReversalId>) -> Self {
        Self {
            credit_reversal: credit_reversal.into(),
            inner: RetrieveTreasuryCreditReversalBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTreasuryCreditReversal {
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

impl StripeRequest for RetrieveTreasuryCreditReversal {
    type Output = stripe_treasury::TreasuryCreditReversal;

    fn build(&self) -> RequestBuilder {
        let credit_reversal = &self.credit_reversal;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/treasury/credit_reversals/{credit_reversal}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTreasuryCreditReversalBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    received_credit: String,
}
impl CreateTreasuryCreditReversalBuilder {
    fn new(received_credit: impl Into<String>) -> Self {
        Self { expand: None, metadata: None, received_credit: received_credit.into() }
    }
}
/// Reverses a ReceivedCredit and creates a CreditReversal object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryCreditReversal {
    inner: CreateTreasuryCreditReversalBuilder,
}
impl CreateTreasuryCreditReversal {
    /// Construct a new `CreateTreasuryCreditReversal`.
    pub fn new(received_credit: impl Into<String>) -> Self {
        Self { inner: CreateTreasuryCreditReversalBuilder::new(received_credit.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl CreateTreasuryCreditReversal {
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

impl StripeRequest for CreateTreasuryCreditReversal {
    type Output = stripe_treasury::TreasuryCreditReversal;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/treasury/credit_reversals").form(&self.inner)
    }
}
