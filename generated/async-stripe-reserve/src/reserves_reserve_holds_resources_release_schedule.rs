#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReservesReserveHoldsResourcesReleaseSchedule {
    /// The time after which the ReserveHold is requested to be released.
    pub release_after: Option<stripe_types::Timestamp>,
    /// The time at which the ReserveHold is scheduled to be released, automatically set to midnight UTC of the day after `release_after`.
    pub scheduled_release: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct ReservesReserveHoldsResourcesReleaseScheduleBuilder {
    release_after: Option<Option<stripe_types::Timestamp>>,
    scheduled_release: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for ReservesReserveHoldsResourcesReleaseSchedule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReservesReserveHoldsResourcesReleaseSchedule>,
        builder: ReservesReserveHoldsResourcesReleaseScheduleBuilder,
    }

    impl Visitor for Place<ReservesReserveHoldsResourcesReleaseSchedule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReservesReserveHoldsResourcesReleaseScheduleBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ReservesReserveHoldsResourcesReleaseScheduleBuilder {
        type Out = ReservesReserveHoldsResourcesReleaseSchedule;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "release_after" => Deserialize::begin(&mut self.release_after),
                "scheduled_release" => Deserialize::begin(&mut self.scheduled_release),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                release_after: Deserialize::default(),
                scheduled_release: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(release_after), Some(scheduled_release)) =
                (self.release_after, self.scheduled_release)
            else {
                return None;
            };
            Some(Self::Out { release_after, scheduled_release })
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

    impl ObjectDeser for ReservesReserveHoldsResourcesReleaseSchedule {
        type Builder = ReservesReserveHoldsResourcesReleaseScheduleBuilder;
    }

    impl FromValueOpt for ReservesReserveHoldsResourcesReleaseSchedule {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ReservesReserveHoldsResourcesReleaseScheduleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "release_after" => b.release_after = FromValueOpt::from_value(v),
                    "scheduled_release" => b.scheduled_release = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
