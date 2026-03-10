#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReservesReservePlansResourcesRollingRelease {
    /// The number of days to reserve funds before releasing.
    pub days_after_charge: u32,
    /// The time at which the ReservePlan expires.
    pub expires_on: Option<i64>,
}
#[doc(hidden)]
pub struct ReservesReservePlansResourcesRollingReleaseBuilder {
    days_after_charge: Option<u32>,
    expires_on: Option<Option<i64>>,
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
                builder: ReservesReservePlansResourcesRollingReleaseBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ReservesReservePlansResourcesRollingReleaseBuilder {
        type Out = ReservesReservePlansResourcesRollingRelease;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "days_after_charge" => Deserialize::begin(&mut self.days_after_charge),
                "expires_on" => Deserialize::begin(&mut self.expires_on),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { days_after_charge: Deserialize::default(), expires_on: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(days_after_charge), Some(expires_on)) =
                (self.days_after_charge, self.expires_on)
            else {
                return None;
            };
            Some(Self::Out { days_after_charge, expires_on })
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

    impl ObjectDeser for ReservesReservePlansResourcesRollingRelease {
        type Builder = ReservesReservePlansResourcesRollingReleaseBuilder;
    }

    impl FromValueOpt for ReservesReservePlansResourcesRollingRelease {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ReservesReservePlansResourcesRollingReleaseBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "days_after_charge" => b.days_after_charge = FromValueOpt::from_value(v),
                    "expires_on" => b.expires_on = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
