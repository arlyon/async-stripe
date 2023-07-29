/// A test clock enables deterministic control over objects in testmode.
///
/// With a test clock, you can create objects at a frozen time in the past or future, and advance to a specific future time to observe webhooks and state changes.
/// After the clock advances, you can either validate the current state of your scenario (and test your assumptions), change the current state of your scenario (and test more complex scenarios), or keep advancing forward in time.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TestClock {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which this clock is scheduled to auto delete.
    pub deletes_after: stripe_types::Timestamp,
    /// Time at which all objects belonging to this clock are frozen.
    pub frozen_time: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_types::test_helpers::test_clock::TestHelpersTestClockId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The custom name supplied at creation.
    pub name: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TestClockObject,
    /// The status of the Test Clock.
    pub status: TestClockStatus,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TestClock {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TestClockObject {
    TestHelpersTestClock,
}

impl TestClockObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TestHelpersTestClock => "test_helpers.test_clock",
        }
    }
}

impl std::str::FromStr for TestClockObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "test_helpers.test_clock" => Ok(Self::TestHelpersTestClock),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TestClockObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TestClockObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TestClockObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TestClockObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TestClockObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TestClockObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TestClockObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TestClockObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The status of the Test Clock.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TestClockStatus {
    Advancing,
    InternalFailure,
    Ready,
}

impl TestClockStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advancing => "advancing",
            Self::InternalFailure => "internal_failure",
            Self::Ready => "ready",
        }
    }
}

impl std::str::FromStr for TestClockStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "advancing" => Ok(Self::Advancing),
            "internal_failure" => Ok(Self::InternalFailure),
            "ready" => Ok(Self::Ready),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TestClockStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TestClockStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TestClockStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TestClockStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TestClockStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TestClockStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TestClockStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TestClock {
    type Id = stripe_types::test_helpers::test_clock::TestHelpersTestClockId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TestHelpersTestClockId, "clock_");
pub mod deleted;
pub use deleted::DeletedTestClock;
