#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourcePauseScheduleResume {
    /// Time at which the subscription resumes.
    pub resume_at: stripe_types::Timestamp,
    pub settings: stripe_shared::SubscriptionSchedulesResourcePauseScheduleResumeSettings,
    pub status: stripe_shared::SubscriptionSchedulesResourcePauseScheduleResumeStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleResume {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourcePauseScheduleResume").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourcePauseScheduleResumeBuilder {
    resume_at: Option<stripe_types::Timestamp>,
    settings: Option<stripe_shared::SubscriptionSchedulesResourcePauseScheduleResumeSettings>,
    status: Option<stripe_shared::SubscriptionSchedulesResourcePauseScheduleResumeStatus>,
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

    impl Deserialize for SubscriptionSchedulesResourcePauseScheduleResume {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourcePauseScheduleResume>,
        builder: SubscriptionSchedulesResourcePauseScheduleResumeBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourcePauseScheduleResume> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionSchedulesResourcePauseScheduleResumeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourcePauseScheduleResumeBuilder {
        type Out = SubscriptionSchedulesResourcePauseScheduleResume;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "resume_at" => Deserialize::begin(&mut self.resume_at),
                "settings" => Deserialize::begin(&mut self.settings),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { resume_at: None, settings: None, status: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(resume_at), Some(settings), Some(status)) =
                (self.resume_at, self.settings.take(), self.status.take())
            else {
                return None;
            };
            Some(Self::Out { resume_at, settings, status })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionSchedulesResourcePauseScheduleResume {
        type Builder = SubscriptionSchedulesResourcePauseScheduleResumeBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourcePauseScheduleResume {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionSchedulesResourcePauseScheduleResumeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "resume_at" => b.resume_at = FromValueOpt::from_value(v),
                    "settings" => b.settings = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
