#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReservesReservePlansResourcesRollingRelease {
    /// The number of days to reserve funds before releasing.
    pub days_after_charge: u32,
    /// The time at which the ReservePlan expires.
    pub expires_on: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReservesReservePlansResourcesRollingRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReservesReservePlansResourcesRollingRelease").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReservesReservePlansResourcesRollingReleaseBuilder {
    days_after_charge: Option<u32>,
    expires_on: Option<Option<i64>>,
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

    impl Deserialize for ReservesReservePlansResourcesRollingRelease {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReservesReservePlansResourcesRollingRelease>,
        builder: ReservesReservePlansResourcesRollingReleaseBuilder,
    }

    impl Visitor for Place<ReservesReservePlansResourcesRollingRelease> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReservesReservePlansResourcesRollingReleaseBuilder {
                    days_after_charge: Deserialize::default(),
                    expires_on: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "days_after_charge" => Deserialize::begin(&mut self.builder.days_after_charge),
                "expires_on" => Deserialize::begin(&mut self.builder.expires_on),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(days_after_charge), Some(expires_on)) =
                (self.builder.days_after_charge, self.builder.expires_on)
            else {
                return Ok(());
            };
            *self.out =
                Some(ReservesReservePlansResourcesRollingRelease { days_after_charge, expires_on });
            Ok(())
        }
    }
};
