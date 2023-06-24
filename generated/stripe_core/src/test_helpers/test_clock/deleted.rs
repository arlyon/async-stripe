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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedTestClockObject {
    TestHelpersTestClock,
}

impl DeletedTestClockObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TestHelpersTestClock => "test_helpers.test_clock",
        }
    }
}

impl std::str::FromStr for DeletedTestClockObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "test_helpers.test_clock" => Ok(Self::TestHelpersTestClock),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedTestClockObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedTestClockObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<DeletedTestClockObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedTestClockObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedTestClock {
    type Id = stripe_core::test_helpers::test_clock::TestHelpersTestClockId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
