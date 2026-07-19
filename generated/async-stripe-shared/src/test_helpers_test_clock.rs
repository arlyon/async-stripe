/// A test clock enables deterministic control over objects in testmode.
/// With a test clock, you can create.
/// objects at a frozen time in the past or future, and advance to a specific future time to observe webhooks and state changes.
/// After the clock advances,.
/// you can either validate the current state of your scenario (and test your assumptions), change the current state of your scenario (and test more complex scenarios), or keep advancing forward in time.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TestHelpersTestClock {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which this clock is scheduled to auto delete.
    pub deletes_after: stripe_types::Timestamp,
    /// Time at which all objects belonging to this clock are frozen.
    pub frozen_time: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_shared::TestHelpersTestClockId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The custom name supplied at creation.
    pub name: Option<String>,
    /// The status of the Test Clock.
    pub status: TestHelpersTestClockStatus,
    pub status_details: stripe_shared::BillingClocksResourceStatusDetailsStatusDetails,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TestHelpersTestClock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TestHelpersTestClock").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TestHelpersTestClockBuilder {
    created: Option<stripe_types::Timestamp>,
    deletes_after: Option<stripe_types::Timestamp>,
    frozen_time: Option<stripe_types::Timestamp>,
    id: Option<stripe_shared::TestHelpersTestClockId>,
    livemode: Option<bool>,
    name: Option<Option<String>>,
    status: Option<TestHelpersTestClockStatus>,
    status_details: Option<stripe_shared::BillingClocksResourceStatusDetailsStatusDetails>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for TestHelpersTestClock {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TestHelpersTestClock>,
        builder: TestHelpersTestClockBuilder,
    }

    impl Visitor for Place<TestHelpersTestClock> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TestHelpersTestClockBuilder {
                    created: Deserialize::default(),
                    deletes_after: Deserialize::default(),
                    frozen_time: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    name: Deserialize::default(),
                    status: Deserialize::default(),
                    status_details: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "deletes_after" => Deserialize::begin(&mut self.builder.deletes_after),
                "frozen_time" => Deserialize::begin(&mut self.builder.frozen_time),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "name" => Deserialize::begin(&mut self.builder.name),
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_details" => Deserialize::begin(&mut self.builder.status_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(created),
                Some(deletes_after),
                Some(frozen_time),
                Some(id),
                Some(livemode),
                Some(name),
                Some(status),
                Some(status_details),
            ) = (
                self.builder.created,
                self.builder.deletes_after,
                self.builder.frozen_time,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.name.take(),
                self.builder.status.take(),
                self.builder.status_details,
            )
            else {
                return Ok(());
            };
            *self.out = Some(TestHelpersTestClock {
                created,
                deletes_after,
                frozen_time,
                id,
                livemode,
                name,
                status,
                status_details,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TestHelpersTestClock {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TestHelpersTestClock", 9)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("deletes_after", &self.deletes_after)?;
        s.serialize_field("frozen_time", &self.frozen_time)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_details", &self.status_details)?;

        s.serialize_field("object", "test_helpers.test_clock")?;
        s.end()
    }
}
/// The status of the Test Clock.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TestHelpersTestClockStatus {
    Advancing,
    InternalFailure,
    Ready,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TestHelpersTestClockStatus {
    pub fn as_str(&self) -> &str {
        use TestHelpersTestClockStatus::*;
        match self {
            Advancing => "advancing",
            InternalFailure => "internal_failure",
            Ready => "ready",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TestHelpersTestClockStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TestHelpersTestClockStatus::*;
        match s {
            "advancing" => Ok(Advancing),
            "internal_failure" => Ok(InternalFailure),
            "ready" => Ok(Ready),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TestHelpersTestClockStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TestHelpersTestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TestHelpersTestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TestHelpersTestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TestHelpersTestClockStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TestHelpersTestClockStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TestHelpersTestClockStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TestHelpersTestClockStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TestHelpersTestClockStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TestHelpersTestClockStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for TestHelpersTestClock {
    type Id = stripe_shared::TestHelpersTestClockId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TestHelpersTestClockId);
