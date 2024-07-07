use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a test clock.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteTestHelpersTestClock<'a> {
    test_clock: &'a stripe_shared::TestHelpersTestClockId,
}
impl<'a> DeleteTestHelpersTestClock<'a> {
    /// Construct a new `DeleteTestHelpersTestClock`.
    pub fn new(test_clock: &'a stripe_shared::TestHelpersTestClockId) -> Self {
        Self { test_clock }
    }
}
impl DeleteTestHelpersTestClock<'_> {
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

impl StripeRequest for DeleteTestHelpersTestClock<'_> {
    type Output = stripe_shared::DeletedTestHelpersTestClock;

    fn build(&self) -> RequestBuilder {
        let test_clock = self.test_clock;
        RequestBuilder::new(StripeMethod::Delete, format!("/test_helpers/test_clocks/{test_clock}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTestHelpersTestClockBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListTestHelpersTestClockBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of your test clocks.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTestHelpersTestClock<'a> {
    inner: ListTestHelpersTestClockBuilder<'a>,
}
impl<'a> ListTestHelpersTestClock<'a> {
    /// Construct a new `ListTestHelpersTestClock`.
    pub fn new() -> Self {
        Self { inner: ListTestHelpersTestClockBuilder::new() }
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
impl<'a> Default for ListTestHelpersTestClock<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTestHelpersTestClock<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::TestHelpersTestClock>>
    {
        stripe_client_core::ListPaginator::new_list("/test_helpers/test_clocks", self.inner)
    }
}

impl StripeRequest for ListTestHelpersTestClock<'_> {
    type Output = stripe_types::List<stripe_shared::TestHelpersTestClock>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/test_helpers/test_clocks").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTestHelpersTestClockBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTestHelpersTestClockBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a test clock.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTestHelpersTestClock<'a> {
    inner: RetrieveTestHelpersTestClockBuilder<'a>,
    test_clock: &'a stripe_shared::TestHelpersTestClockId,
}
impl<'a> RetrieveTestHelpersTestClock<'a> {
    /// Construct a new `RetrieveTestHelpersTestClock`.
    pub fn new(test_clock: &'a stripe_shared::TestHelpersTestClockId) -> Self {
        Self { test_clock, inner: RetrieveTestHelpersTestClockBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTestHelpersTestClock<'_> {
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

impl StripeRequest for RetrieveTestHelpersTestClock<'_> {
    type Output = stripe_shared::TestHelpersTestClock;

    fn build(&self) -> RequestBuilder {
        let test_clock = self.test_clock;
        RequestBuilder::new(StripeMethod::Get, format!("/test_helpers/test_clocks/{test_clock}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTestHelpersTestClockBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    frozen_time: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}
impl<'a> CreateTestHelpersTestClockBuilder<'a> {
    fn new(frozen_time: stripe_types::Timestamp) -> Self {
        Self { expand: None, frozen_time, name: None }
    }
}
/// Creates a new test clock that can be attached to new customers and quotes.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTestHelpersTestClock<'a> {
    inner: CreateTestHelpersTestClockBuilder<'a>,
}
impl<'a> CreateTestHelpersTestClock<'a> {
    /// Construct a new `CreateTestHelpersTestClock`.
    pub fn new(frozen_time: stripe_types::Timestamp) -> Self {
        Self { inner: CreateTestHelpersTestClockBuilder::new(frozen_time) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The name for this test clock.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
}
impl CreateTestHelpersTestClock<'_> {
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

impl StripeRequest for CreateTestHelpersTestClock<'_> {
    type Output = stripe_shared::TestHelpersTestClock;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/test_helpers/test_clocks").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct AdvanceTestHelpersTestClockBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    frozen_time: stripe_types::Timestamp,
}
impl<'a> AdvanceTestHelpersTestClockBuilder<'a> {
    fn new(frozen_time: stripe_types::Timestamp) -> Self {
        Self { expand: None, frozen_time }
    }
}
/// Starts advancing a test clock to a specified time in the future.
/// Advancement is done when status changes to `Ready`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct AdvanceTestHelpersTestClock<'a> {
    inner: AdvanceTestHelpersTestClockBuilder<'a>,
    test_clock: &'a stripe_shared::TestHelpersTestClockId,
}
impl<'a> AdvanceTestHelpersTestClock<'a> {
    /// Construct a new `AdvanceTestHelpersTestClock`.
    pub fn new(
        test_clock: &'a stripe_shared::TestHelpersTestClockId,
        frozen_time: stripe_types::Timestamp,
    ) -> Self {
        Self { test_clock, inner: AdvanceTestHelpersTestClockBuilder::new(frozen_time) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl AdvanceTestHelpersTestClock<'_> {
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

impl StripeRequest for AdvanceTestHelpersTestClock<'_> {
    type Output = stripe_shared::TestHelpersTestClock;

    fn build(&self) -> RequestBuilder {
        let test_clock = self.test_clock;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/test_clocks/{test_clock}/advance"),
        )
        .form(&self.inner)
    }
}
