use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListScheduledQueryRunBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListScheduledQueryRunBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of scheduled query runs.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListScheduledQueryRun {
    inner: ListScheduledQueryRunBuilder,
}
impl ListScheduledQueryRun {
    /// Construct a new `ListScheduledQueryRun`.
    pub fn new() -> Self {
        Self { inner: ListScheduledQueryRunBuilder::new() }
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListScheduledQueryRun {
    fn default() -> Self {
        Self::new()
    }
}
impl ListScheduledQueryRun {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::ScheduledQueryRun>> {
        stripe_client_core::ListPaginator::new_list("/sigma/scheduled_query_runs", &self.inner)
    }
}

impl StripeRequest for ListScheduledQueryRun {
    type Output = stripe_types::List<stripe_misc::ScheduledQueryRun>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/sigma/scheduled_query_runs").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveScheduledQueryRunBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveScheduledQueryRunBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an scheduled query run.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveScheduledQueryRun {
    inner: RetrieveScheduledQueryRunBuilder,
    scheduled_query_run: String,
}
impl RetrieveScheduledQueryRun {
    /// Construct a new `RetrieveScheduledQueryRun`.
    pub fn new(scheduled_query_run: impl Into<String>) -> Self {
        Self {
            scheduled_query_run: scheduled_query_run.into(),
            inner: RetrieveScheduledQueryRunBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveScheduledQueryRun {
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

impl StripeRequest for RetrieveScheduledQueryRun {
    type Output = stripe_misc::ScheduledQueryRun;

    fn build(&self) -> RequestBuilder {
        let scheduled_query_run = &self.scheduled_query_run;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/sigma/scheduled_query_runs/{scheduled_query_run}"),
        )
        .query(&self.inner)
    }
}
