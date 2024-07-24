use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListScheduledQueryRunBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListScheduledQueryRunBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of scheduled query runs.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListScheduledQueryRun<'a> {
    inner: ListScheduledQueryRunBuilder<'a>,
}
impl<'a> ListScheduledQueryRun<'a> {
    /// Construct a new `ListScheduledQueryRun`.
    pub fn new() -> Self {
        Self { inner: ListScheduledQueryRunBuilder::new() }
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListScheduledQueryRun<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListScheduledQueryRun<'_> {
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
        stripe_client_core::ListPaginator::new_list("/sigma/scheduled_query_runs", self.inner)
    }
}

impl StripeRequest for ListScheduledQueryRun<'_> {
    type Output = stripe_types::List<stripe_misc::ScheduledQueryRun>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/sigma/scheduled_query_runs").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveScheduledQueryRunBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveScheduledQueryRunBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an scheduled query run.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveScheduledQueryRun<'a> {
    inner: RetrieveScheduledQueryRunBuilder<'a>,
    scheduled_query_run: &'a str,
}
impl<'a> RetrieveScheduledQueryRun<'a> {
    /// Construct a new `RetrieveScheduledQueryRun`.
    pub fn new(scheduled_query_run: &'a str) -> Self {
        Self { scheduled_query_run, inner: RetrieveScheduledQueryRunBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveScheduledQueryRun<'_> {
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

impl StripeRequest for RetrieveScheduledQueryRun<'_> {
    type Output = stripe_misc::ScheduledQueryRun;

    fn build(&self) -> RequestBuilder {
        let scheduled_query_run = self.scheduled_query_run;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/sigma/scheduled_query_runs/{scheduled_query_run}"),
        )
        .query(&self.inner)
    }
}
