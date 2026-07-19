#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReservesReservePlansResourcesFixedRelease {
    /// The time after which all reserved funds are requested for release.
    pub release_after: i64,
    /// The time at which reserved funds are scheduled for release, automatically set to midnight UTC of the day after `release_after`.
    pub scheduled_release: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReservesReservePlansResourcesFixedRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReservesReservePlansResourcesFixedRelease").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReservesReservePlansResourcesFixedReleaseBuilder {
    release_after: Option<i64>,
    scheduled_release: Option<stripe_types::Timestamp>,
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

    impl Deserialize for ReservesReservePlansResourcesFixedRelease {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReservesReservePlansResourcesFixedRelease>,
        builder: ReservesReservePlansResourcesFixedReleaseBuilder,
    }

    impl Visitor for Place<ReservesReservePlansResourcesFixedRelease> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReservesReservePlansResourcesFixedReleaseBuilder {
                    release_after: Deserialize::default(),
                    scheduled_release: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "release_after" => Deserialize::begin(&mut self.builder.release_after),
                "scheduled_release" => Deserialize::begin(&mut self.builder.scheduled_release),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(release_after), Some(scheduled_release)) =
                (self.builder.release_after, self.builder.scheduled_release)
            else {
                return Ok(());
            };
            *self.out = Some(ReservesReservePlansResourcesFixedRelease {
                release_after,
                scheduled_release,
            });
            Ok(())
        }
    }
};
