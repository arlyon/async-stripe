// pub struct TestClockRetrieve {}

use serde::Serialize;

use crate::{
    params::Paginable, Client, List, Response, TestHelpersTestClock, TestHelpersTestClockId,
    Timestamp,
};

#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateTestClock<'a> {
    /// The initial frozen time for this test clock.
    pub frozen_time: Timestamp,

    /// The name for this test clock.
    pub name: &'a str,
}

impl<'a> CreateTestClock<'a> {
    pub fn new() -> Self {
        Self { frozen_time: Default::default(), name: Default::default() }
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct AdvanceTestClock {
    /// The time to advance the test clock. Must be after the test clock’s current frozen time.
    /// Cannot be more than two intervals in the future from the shortest subscription in this test clock.
    /// If there are no subscriptions in this test clock, it cannot be more than two years in the future.
    pub frozen_time: Timestamp,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct ListTestClocks {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list. For instance,
    /// if you make a list request and receive 100 objects, starting with `obj_bar`,
    /// your subsequent call can include `ending_before=obj_bar` in order to fetch the previous
    /// page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TestHelpersTestClockId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list. For instance,
    /// if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent
    /// call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<TestHelpersTestClockId>,

    /// A limit on the number of objects to be returned. Limit can range between 1 and 100,
    /// and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

impl ListTestClocks {
    pub fn new() -> Self {
        Self {
            ending_before: Default::default(),
            starting_after: Default::default(),
            limit: Default::default(),
        }
    }
}

impl Paginable for ListTestClocks {
    type O = TestHelpersTestClock;

    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id);
    }
}

impl TestHelpersTestClock {
    /// Creates a new test clock that can be attached to new customers and quotes.
    ///
    /// For more details see <https://docs.stripe.com/api/test_clocks/create>
    pub fn create(client: &Client, params: &CreateTestClock<'_>) -> Response<TestHelpersTestClock> {
        client.post_form("/test_helpers/test_clocks", params)
    }

    /// Retrieves a test clock.
    ///
    /// For more details see <https://docs.stripe.com/api/test_clocks/retrieve>
    pub fn retrieve(
        client: &Client,
        id: &TestHelpersTestClockId,
    ) -> Response<TestHelpersTestClock> {
        client.get(&format!("/test_helpers/test_clocks/{}", id))
    }

    /// Returns a list of your test clocks.
    ///
    /// For more details see <https://docs.stripe.com/api/test_clocks/list>
    pub fn list(client: &Client, params: &ListTestClocks) -> Response<List<TestHelpersTestClock>> {
        client.get_query("/test_helpers/test_clocks", params)
    }

    /// Deletes a test clock.
    ///
    /// For more details see <https://docs.stripe.com/api/test_clocks/delete>
    pub fn delete(client: &Client, id: &TestHelpersTestClockId) -> Response<TestHelpersTestClock> {
        client.delete(&format!("/test_helpers/test_clocks/{}", id))
    }

    /// Starts advancing a test clock to a specified time in the future. Advancement is done when status changes to `Ready`.
    ///
    /// For more details see <https://docs.stripe.com/api/test_clocks/advance>
    pub fn advance(
        client: &Client,
        test_clock_id: &TestHelpersTestClockId,
        params: &AdvanceTestClock,
    ) -> Response<TestHelpersTestClock> {
        client.post_form(&format!("test_helpers/test_clocks/{}/advance", test_clock_id), params)
    }
}
