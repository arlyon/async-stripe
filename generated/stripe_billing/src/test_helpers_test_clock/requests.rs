#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteTestHelpersTestClock {}
impl DeleteTestHelpersTestClock {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteTestHelpersTestClock {
    /// Deletes a test clock.
    pub fn send(
        &self,
        client: &stripe::Client,
        test_clock: &stripe_shared::TestHelpersTestClockId,
    ) -> stripe::Response<stripe_shared::DeletedTestHelpersTestClock> {
        client.send_form(
            &format!("/test_helpers/test_clocks/{test_clock}"),
            self,
            http_types::Method::Delete,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTestHelpersTestClock<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListTestHelpersTestClock<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTestHelpersTestClock<'a> {
    /// Returns a list of your test clocks.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::TestHelpersTestClock>> {
        client.get_query("/test_helpers/test_clocks", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_shared::TestHelpersTestClock>> {
        stripe::ListPaginator::from_list_params("/test_helpers/test_clocks", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTestHelpersTestClock<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTestHelpersTestClock<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTestHelpersTestClock<'a> {
    /// Retrieves a test clock.
    pub fn send(
        &self,
        client: &stripe::Client,
        test_clock: &stripe_shared::TestHelpersTestClockId,
    ) -> stripe::Response<stripe_shared::TestHelpersTestClock> {
        client.get_query(&format!("/test_helpers/test_clocks/{test_clock}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTestHelpersTestClock<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The initial frozen time for this test clock.
    pub frozen_time: stripe_types::Timestamp,
    /// The name for this test clock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}
impl<'a> CreateTestHelpersTestClock<'a> {
    pub fn new(frozen_time: stripe_types::Timestamp) -> Self {
        Self { expand: None, frozen_time, name: None }
    }
}
impl<'a> CreateTestHelpersTestClock<'a> {
    /// Creates a new test clock that can be attached to new customers and quotes.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_shared::TestHelpersTestClock> {
        client.send_form("/test_helpers/test_clocks", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AdvanceTestHelpersTestClock<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The time to advance the test clock.
    /// Must be after the test clock's current frozen time.
    /// Cannot be more than two intervals in the future from the shortest subscription in this test clock.
    /// If there are no subscriptions in this test clock, it cannot be more than two years in the future.
    pub frozen_time: stripe_types::Timestamp,
}
impl<'a> AdvanceTestHelpersTestClock<'a> {
    pub fn new(frozen_time: stripe_types::Timestamp) -> Self {
        Self { expand: None, frozen_time }
    }
}
impl<'a> AdvanceTestHelpersTestClock<'a> {
    /// Starts advancing a test clock to a specified time in the future.
    /// Advancement is done when status changes to `Ready`.
    pub fn send(
        &self,
        client: &stripe::Client,
        test_clock: &stripe_shared::TestHelpersTestClockId,
    ) -> stripe::Response<stripe_shared::TestHelpersTestClock> {
        client.send_form(
            &format!("/test_helpers/test_clocks/{test_clock}/advance"),
            self,
            http_types::Method::Post,
        )
    }
}