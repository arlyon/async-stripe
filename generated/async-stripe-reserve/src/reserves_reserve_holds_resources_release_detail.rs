#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReservesReserveHoldsResourcesReleaseDetail {
    /// The amount released by the ReserveRelease from this ReserveHold.
    /// A positive integer representing how much is released in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount: i64,
    /// The ReserveRelease which released funds from this ReserveHold (e.g., resrel_123).
    pub reserve_release: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReservesReserveHoldsResourcesReleaseDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReservesReserveHoldsResourcesReleaseDetail").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReservesReserveHoldsResourcesReleaseDetailBuilder {
    amount: Option<i64>,
    reserve_release: Option<String>,
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

    impl Deserialize for ReservesReserveHoldsResourcesReleaseDetail {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReservesReserveHoldsResourcesReleaseDetail>,
        builder: ReservesReserveHoldsResourcesReleaseDetailBuilder,
    }

    impl Visitor for Place<ReservesReserveHoldsResourcesReleaseDetail> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReservesReserveHoldsResourcesReleaseDetailBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ReservesReserveHoldsResourcesReleaseDetailBuilder {
        type Out = ReservesReserveHoldsResourcesReleaseDetail;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "reserve_release" => Deserialize::begin(&mut self.reserve_release),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount: None, reserve_release: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(reserve_release)) = (self.amount, self.reserve_release.take())
            else {
                return None;
            };
            Some(Self::Out { amount, reserve_release })
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

    impl ObjectDeser for ReservesReserveHoldsResourcesReleaseDetail {
        type Builder = ReservesReserveHoldsResourcesReleaseDetailBuilder;
    }

    impl FromValueOpt for ReservesReserveHoldsResourcesReleaseDetail {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ReservesReserveHoldsResourcesReleaseDetailBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "reserve_release" => b.reserve_release = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
