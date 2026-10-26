#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourcePauseScheduleStatusError {
    /// A machine-readable error code.
    pub code: Option<String>,
    /// A description of the error.
    pub message: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourcePauseScheduleStatusError")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourcePauseScheduleStatusErrorBuilder {
    code: Option<Option<String>>,
    message: Option<String>,
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

    impl Deserialize for SubscriptionSchedulesResourcePauseScheduleStatusError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourcePauseScheduleStatusError>,
        builder: SubscriptionSchedulesResourcePauseScheduleStatusErrorBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourcePauseScheduleStatusError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    SubscriptionSchedulesResourcePauseScheduleStatusErrorBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourcePauseScheduleStatusErrorBuilder {
        type Out = SubscriptionSchedulesResourcePauseScheduleStatusError;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "message" => Deserialize::begin(&mut self.message),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: Some(None), message: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(message)) = (self.code.take(), self.message.take()) else {
                return None;
            };
            Some(Self::Out { code, message })
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

    impl ObjectDeser for SubscriptionSchedulesResourcePauseScheduleStatusError {
        type Builder = SubscriptionSchedulesResourcePauseScheduleStatusErrorBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourcePauseScheduleStatusError {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                SubscriptionSchedulesResourcePauseScheduleStatusErrorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "message" => b.message = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
