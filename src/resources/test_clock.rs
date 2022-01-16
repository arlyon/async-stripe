use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::TestClockId;
use crate::params::{Deleted, Expand, List, Timestamp};

// TODO: When the openapi spec includes test_clock, crate::resources::test_clock
// may be removed in favor of the generated version.

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestClock {
    /// Unique identifier for the object.
    pub id: TestClockId,

    /// String representing the object’s type. Objects of the same type share
    /// the same value.
    pub object: String,

    /// Time at which the object was created. Measured in seconds since the Unix
    /// epoch.
    pub created: Timestamp,

    /// Time at which all objects belonging to this clock are frozen.
    pub frozen_time: Timestamp,

    /// Has the value true if the object exists in live mode or the value false
    /// if the object exists in test mode.
    pub livemode: bool,

    /// The custom name supplied at creation.
    pub name: Option<String>,

    /// The status of the Test Clock. The status is `ready` if all objects have
    /// advanced to the frozen time, or `advancing` if the objects are still
    /// being processed.
    pub status: Status,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ready,
    Advancing,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateTestClock<'a> {
    /// The initial frozen time of this test clock.
    pub frozen_time: Timestamp,

    /// The name for this test clock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct ListTestClocks {
    /// A cursor for use in pagination.
    ///
    /// ending_before is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with obj_bar, your subsequent call can include ending_before=obj_bar in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TestClockId>,

    /// A limit on the number of objects to be returned. Limit can range between
    /// 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// starting_after is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with obj_foo, your subsequent call can include starting_after=obj_foo in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<TestClockId>,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct AdvanceTestClock {
    /// The initial frozen time of this test clock.
    pub frozen_time: Timestamp,
}

impl TestClock {
    pub fn create(client: &Client, params: &CreateTestClock<'_>) -> Response<TestClock> {
        client.post_form("/test_helpers/test_clocks", &params)
    }

    pub fn list(client: &Client, params: &ListTestClocks) -> Response<List<TestClock>> {
        client.get_query("/test_helpers/test_clocks", &params)
    }

    pub fn retrieve(client: &Client, id: &TestClockId) -> Response<TestClock> {
        client.get_query(&format!("/test_helpers/test_clocks/{}", id), &Expand { expand: &[] })
    }

    pub fn delete(client: &Client, id: &TestClockId) -> Response<Deleted<TestClockId>> {
        client.delete(&format!("/test_helpers/test_clocks/{}", id))
    }

    pub fn advance(
        client: &Client,
        id: &TestClockId,
        params: &AdvanceTestClock,
    ) -> Response<TestClock> {
        client.post_form(&format!("/test_helpers/test_clocks/{}/advance", id), params)
    }
}
