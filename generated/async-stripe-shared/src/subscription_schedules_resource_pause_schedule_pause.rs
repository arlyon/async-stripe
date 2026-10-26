#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourcePauseSchedulePause {
    /// Time at which the subscription pauses.
    pub pause_at: stripe_types::Timestamp,
    /// Settings controlling billing behavior during the pause.
    pub settings: Option<stripe_shared::SubscriptionSchedulesResourcePauseSchedulePauseSettings>,
    pub status: stripe_shared::SubscriptionSchedulesResourcePauseSchedulePauseStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseSchedulePause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourcePauseSchedulePause").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourcePauseSchedulePauseBuilder {
    pause_at: Option<stripe_types::Timestamp>,
    settings:
        Option<Option<stripe_shared::SubscriptionSchedulesResourcePauseSchedulePauseSettings>>,
    status: Option<stripe_shared::SubscriptionSchedulesResourcePauseSchedulePauseStatus>,
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

    impl Deserialize for SubscriptionSchedulesResourcePauseSchedulePause {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourcePauseSchedulePause>,
        builder: SubscriptionSchedulesResourcePauseSchedulePauseBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourcePauseSchedulePause> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionSchedulesResourcePauseSchedulePauseBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourcePauseSchedulePauseBuilder {
        type Out = SubscriptionSchedulesResourcePauseSchedulePause;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "pause_at" => Deserialize::begin(&mut self.pause_at),
                "settings" => Deserialize::begin(&mut self.settings),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { pause_at: None, settings: Some(None), status: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(pause_at), Some(settings), Some(status)) =
                (self.pause_at, self.settings.take(), self.status.take())
            else {
                return None;
            };
            Some(Self::Out { pause_at, settings, status })
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

    impl ObjectDeser for SubscriptionSchedulesResourcePauseSchedulePause {
        type Builder = SubscriptionSchedulesResourcePauseSchedulePauseBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourcePauseSchedulePause {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionSchedulesResourcePauseSchedulePauseBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "pause_at" => b.pause_at = FromValueOpt::from_value(v),
                    "settings" => b.settings = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
