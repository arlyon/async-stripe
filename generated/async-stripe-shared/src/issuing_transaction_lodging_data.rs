#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionLodgingData {
    /// The time of checking into the lodging.
    pub check_in_at: Option<i64>,
    /// The number of nights stayed at the lodging.
    pub nights: Option<i64>,
}
#[doc(hidden)]
pub struct IssuingTransactionLodgingDataBuilder {
    check_in_at: Option<Option<i64>>,
    nights: Option<Option<i64>>,
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

    impl Deserialize for IssuingTransactionLodgingData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionLodgingData>,
        builder: IssuingTransactionLodgingDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionLodgingData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionLodgingDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionLodgingDataBuilder {
        type Out = IssuingTransactionLodgingData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "check_in_at" => Deserialize::begin(&mut self.check_in_at),
                "nights" => Deserialize::begin(&mut self.nights),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { check_in_at: Deserialize::default(), nights: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(check_in_at), Some(nights)) = (self.check_in_at, self.nights) else {
                return None;
            };
            Some(Self::Out { check_in_at, nights })
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

    impl ObjectDeser for IssuingTransactionLodgingData {
        type Builder = IssuingTransactionLodgingDataBuilder;
    }

    impl FromValueOpt for IssuingTransactionLodgingData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionLodgingDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "check_in_at" => b.check_in_at = FromValueOpt::from_value(v),
                    "nights" => b.nights = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
