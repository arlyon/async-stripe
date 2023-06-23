#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedTestClock {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_core::test_helpers::test_clock::TestHelpersTestClockId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedTestClockObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedTestClock {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DeletedTestClockObject {
    #[serde(rename = "test_helpers.test_clock")]
    TestHelpersTestClock,
}

impl DeletedTestClockObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TestHelpersTestClock => "test_helpers.test_clock",
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
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for DeletedTestClock {
    type Id = stripe_core::test_helpers::test_clock::TestHelpersTestClockId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
