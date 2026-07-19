#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionScheduleCurrentPhase {
    /// The end of this phase of the subscription schedule.
    pub end_date: stripe_types::Timestamp,
    /// The start of this phase of the subscription schedule.
    pub start_date: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionScheduleCurrentPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionScheduleCurrentPhase").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionScheduleCurrentPhaseBuilder {
    end_date: Option<stripe_types::Timestamp>,
    start_date: Option<stripe_types::Timestamp>,
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

    impl Deserialize for SubscriptionScheduleCurrentPhase {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionScheduleCurrentPhase>,
        builder: SubscriptionScheduleCurrentPhaseBuilder,
    }

    impl Visitor for Place<SubscriptionScheduleCurrentPhase> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionScheduleCurrentPhaseBuilder {
                    end_date: Deserialize::default(),
                    start_date: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end_date" => Deserialize::begin(&mut self.builder.end_date),
                "start_date" => Deserialize::begin(&mut self.builder.start_date),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(end_date), Some(start_date)) =
                (self.builder.end_date, self.builder.start_date)
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionScheduleCurrentPhase { end_date, start_date });
            Ok(())
        }
    }
};
