#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Period {
    /// The end date of this usage period. All usage up to and including this point in time is included.
    pub end: Option<stripe_types::Timestamp>,
    /// The start date of this usage period. All usage after this point in time is included.
    pub start: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct PeriodBuilder {
    end: Option<Option<stripe_types::Timestamp>>,
    start: Option<Option<stripe_types::Timestamp>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Period {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Period>,
        builder: PeriodBuilder,
    }

    impl Visitor for Place<Period> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PeriodBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PeriodBuilder {
        type Out = Period;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end" => Deserialize::begin(&mut self.end),
                "start" => Deserialize::begin(&mut self.start),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { end: Deserialize::default(), start: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(end), Some(start)) = (self.end, self.start) else {
                return None;
            };
            Some(Self::Out { end, start })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Period {
        type Builder = PeriodBuilder;
    }

    impl FromValueOpt for Period {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PeriodBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "end" => b.end = FromValueOpt::from_value(v),
                    "start" => b.start = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
