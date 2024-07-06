/// Point in Time
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoDataDocumentReportIssuedDate {
    /// Numerical day between 1 and 31.
    pub day: Option<i64>,
    /// Numerical month between 1 and 12.
    pub month: Option<i64>,
    /// The four-digit year.
    pub year: Option<i64>,
}
#[doc(hidden)]
pub struct GelatoDataDocumentReportIssuedDateBuilder {
    day: Option<Option<i64>>,
    month: Option<Option<i64>>,
    year: Option<Option<i64>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoDataDocumentReportIssuedDate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoDataDocumentReportIssuedDate>,
        builder: GelatoDataDocumentReportIssuedDateBuilder,
    }

    impl Visitor for Place<GelatoDataDocumentReportIssuedDate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoDataDocumentReportIssuedDateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoDataDocumentReportIssuedDateBuilder {
        type Out = GelatoDataDocumentReportIssuedDate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "day" => Deserialize::begin(&mut self.day),
                "month" => Deserialize::begin(&mut self.month),
                "year" => Deserialize::begin(&mut self.year),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                day: Deserialize::default(),
                month: Deserialize::default(),
                year: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { day: self.day?, month: self.month?, year: self.year? })
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

    impl ObjectDeser for GelatoDataDocumentReportIssuedDate {
        type Builder = GelatoDataDocumentReportIssuedDateBuilder;
    }

    impl FromValueOpt for GelatoDataDocumentReportIssuedDate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoDataDocumentReportIssuedDateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "day" => b.day = Some(FromValueOpt::from_value(v)?),
                    "month" => b.month = Some(FromValueOpt::from_value(v)?),
                    "year" => b.year = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
