// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::TestHelpersTestClockId;
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TestClock".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TestHelpersTestClock {
    /// Unique identifier for the object.
    pub id: TestHelpersTestClockId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Time at which this clock is scheduled to auto delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes_after: Option<Timestamp>,

    /// Time at which all objects belonging to this clock are frozen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_time: Option<Timestamp>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// The custom name supplied at creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The status of the Test Clock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TestHelpersTestClockStatus>,
}

impl Object for TestHelpersTestClock {
    type Id = TestHelpersTestClockId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "test_helpers.test_clock"
    }
}

/// An enum representing the possible values of an `TestHelpersTestClock`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TestHelpersTestClockStatus {
    Advancing,
    InternalFailure,
    Ready,
}

impl TestHelpersTestClockStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TestHelpersTestClockStatus::Advancing => "advancing",
            TestHelpersTestClockStatus::InternalFailure => "internal_failure",
            TestHelpersTestClockStatus::Ready => "ready",
        }
    }
}

impl AsRef<str> for TestHelpersTestClockStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TestHelpersTestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TestHelpersTestClockStatus {
    fn default() -> Self {
        Self::Advancing
    }
}
