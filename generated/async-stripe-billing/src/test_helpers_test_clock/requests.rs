use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a test clock.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct DeleteTestHelpersTestClock {
    test_clock: stripe_shared::TestHelpersTestClockId,
}
impl DeleteTestHelpersTestClock {
    /// Construct a new `DeleteTestHelpersTestClock`.
    pub fn new(test_clock: impl Into<stripe_shared::TestHelpersTestClockId>) -> Self {
        Self { test_clock: test_clock.into() }
    }
}
impl DeleteTestHelpersTestClock {
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

impl StripeRequest for DeleteTestHelpersTestClock {
    type Output = stripe_shared::DeletedTestHelpersTestClock;

    fn build(&self) -> RequestBuilder {
        let test_clock = &self.test_clock;
        RequestBuilder::new(StripeMethod::Delete, format!("/test_helpers/test_clocks/{test_clock}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListTestHelpersTestClockBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListTestHelpersTestClockBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of your test clocks.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListTestHelpersTestClock {
    inner: ListTestHelpersTestClockBuilder,
}
impl ListTestHelpersTestClock {
    /// Construct a new `ListTestHelpersTestClock`.
    pub fn new() -> Self {
        Self { inner: ListTestHelpersTestClockBuilder::new() }
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
impl Default for ListTestHelpersTestClock {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTestHelpersTestClock {
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
        stripe_client_core::ListPaginator::new_list("/test_helpers/test_clocks", &self.inner)
    }
}

impl StripeRequest for ListTestHelpersTestClock {
    type Output = stripe_types::List<stripe_shared::TestHelpersTestClock>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/test_helpers/test_clocks").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveTestHelpersTestClockBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTestHelpersTestClockBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a test clock.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveTestHelpersTestClock {
    inner: RetrieveTestHelpersTestClockBuilder,
    test_clock: stripe_shared::TestHelpersTestClockId,
}
impl RetrieveTestHelpersTestClock {
    /// Construct a new `RetrieveTestHelpersTestClock`.
    pub fn new(test_clock: impl Into<stripe_shared::TestHelpersTestClockId>) -> Self {
        Self { test_clock: test_clock.into(), inner: RetrieveTestHelpersTestClockBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTestHelpersTestClock {
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

impl StripeRequest for RetrieveTestHelpersTestClock {
    type Output = stripe_shared::TestHelpersTestClock;

    fn build(&self) -> RequestBuilder {
        let test_clock = &self.test_clock;
        RequestBuilder::new(StripeMethod::Get, format!("/test_helpers/test_clocks/{test_clock}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateTestHelpersTestClockBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    frozen_time: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
impl CreateTestHelpersTestClockBuilder {
    fn new(frozen_time: impl Into<stripe_types::Timestamp>) -> Self {
        Self { expand: None, frozen_time: frozen_time.into(), name: None }
    }
}
/// Creates a new test clock that can be attached to new customers and quotes.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTestHelpersTestClock {
    inner: CreateTestHelpersTestClockBuilder,
}
impl CreateTestHelpersTestClock {
    /// Construct a new `CreateTestHelpersTestClock`.
    pub fn new(frozen_time: impl Into<stripe_types::Timestamp>) -> Self {
        Self { inner: CreateTestHelpersTestClockBuilder::new(frozen_time.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The name for this test clock.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
}
impl CreateTestHelpersTestClock {
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

impl StripeRequest for CreateTestHelpersTestClock {
    type Output = stripe_shared::TestHelpersTestClock;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/test_helpers/test_clocks").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct AdvanceTestHelpersTestClockBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    frozen_time: stripe_types::Timestamp,
}
impl AdvanceTestHelpersTestClockBuilder {
    fn new(frozen_time: impl Into<stripe_types::Timestamp>) -> Self {
        Self { expand: None, frozen_time: frozen_time.into() }
    }
}
/// Starts advancing a test clock to a specified time in the future.
/// Advancement is done when status changes to `Ready`.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct AdvanceTestHelpersTestClock {
    inner: AdvanceTestHelpersTestClockBuilder,
    test_clock: stripe_shared::TestHelpersTestClockId,
}
impl AdvanceTestHelpersTestClock {
    /// Construct a new `AdvanceTestHelpersTestClock`.
    pub fn new(
        test_clock: impl Into<stripe_shared::TestHelpersTestClockId>,
        frozen_time: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self {
            test_clock: test_clock.into(),
            inner: AdvanceTestHelpersTestClockBuilder::new(frozen_time.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl AdvanceTestHelpersTestClock {
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

impl StripeRequest for AdvanceTestHelpersTestClock {
    type Output = stripe_shared::TestHelpersTestClock;

    fn build(&self) -> RequestBuilder {
        let test_clock = &self.test_clock;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/test_clocks/{test_clock}/advance"),
        )
        .form(&self.inner)
    }
}
