#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourcePauseSchedule {
    /// A unique identifier for this pause schedule.
    pub key: String,
    pub pause: stripe_shared::SubscriptionSchedulesResourcePauseSchedulePause,
    /// Details about when and how the subscription resumes.
    pub resume: Option<stripe_shared::SubscriptionSchedulesResourcePauseScheduleResume>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourcePauseSchedule").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourcePauseScheduleBuilder {
    key: Option<String>,
    pause: Option<stripe_shared::SubscriptionSchedulesResourcePauseSchedulePause>,
    resume: Option<Option<stripe_shared::SubscriptionSchedulesResourcePauseScheduleResume>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionSchedulesResourcePauseSchedule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourcePauseSchedule>,
        builder: SubscriptionSchedulesResourcePauseScheduleBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourcePauseSchedule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionSchedulesResourcePauseScheduleBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourcePauseScheduleBuilder {
        type Out = SubscriptionSchedulesResourcePauseSchedule;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "key" => Deserialize::begin(&mut self.key),
                "pause" => Deserialize::begin(&mut self.pause),
                "resume" => Deserialize::begin(&mut self.resume),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { key: None, pause: None, resume: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(key), Some(pause), Some(resume)) =
                (self.key.take(), self.pause.take(), self.resume.take())
            else {
                return None;
            };
            Some(Self::Out { key, pause, resume })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionSchedulesResourcePauseSchedule {
        type Builder = SubscriptionSchedulesResourcePauseScheduleBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourcePauseSchedule {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionSchedulesResourcePauseScheduleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "key" => b.key = FromValueOpt::from_value(v),
                    "pause" => b.pause = FromValueOpt::from_value(v),
                    "resume" => b.resume = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
