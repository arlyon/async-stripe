/// Point in Time
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoDataVerifiedOutputsDate {
    /// Numerical day between 1 and 31.
    pub day: Option<i64>,
    /// Numerical month between 1 and 12.
    pub month: Option<i64>,
    /// The four-digit year.
    pub year: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoDataVerifiedOutputsDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoDataVerifiedOutputsDate").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoDataVerifiedOutputsDateBuilder {
    day: Option<Option<i64>>,
    month: Option<Option<i64>>,
    year: Option<Option<i64>>,
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

    impl Deserialize for GelatoDataVerifiedOutputsDate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoDataVerifiedOutputsDate>,
        builder: GelatoDataVerifiedOutputsDateBuilder,
    }

    impl Visitor for Place<GelatoDataVerifiedOutputsDate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoDataVerifiedOutputsDateBuilder {
                    day: Deserialize::default(),
                    month: Deserialize::default(),
                    year: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "day" => Deserialize::begin(&mut self.builder.day),
                "month" => Deserialize::begin(&mut self.builder.month),
                "year" => Deserialize::begin(&mut self.builder.year),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(day), Some(month), Some(year)) =
                (self.builder.day, self.builder.month, self.builder.year)
            else {
                return Ok(());
            };
            *self.out = Some(GelatoDataVerifiedOutputsDate { day, month, year });
            Ok(())
        }
    }
};
