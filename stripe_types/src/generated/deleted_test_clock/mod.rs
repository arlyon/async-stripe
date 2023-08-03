#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedTestClock {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::test_clock::TestHelpersTestClockId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedTestClockObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DeletedTestClockObject {
    TestHelpersTestClock,
}

impl DeletedTestClockObject {
    pub fn as_str(self) -> &'static str {
        use DeletedTestClockObject::*;
        match self {
            TestHelpersTestClock => "test_helpers.test_clock",
        }
    }
}

impl std::str::FromStr for DeletedTestClockObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedTestClockObject::*;
        match s {
            "test_helpers.test_clock" => Ok(TestHelpersTestClock),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedTestClockObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedTestClockObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DeletedTestClockObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DeletedTestClockObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedTestClockObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for DeletedTestClockObject"))
    }
}
impl stripe_types::Object for DeletedTestClock {
    type Id = stripe_types::test_clock::TestHelpersTestClockId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
