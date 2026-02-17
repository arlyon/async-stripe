#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationFleetFuelPriceData {
    /// Gross fuel amount that should equal Fuel Quantity multiplied by Fuel Unit Cost, inclusive of taxes.
    pub gross_amount_decimal: Option<String>,
}
#[doc(hidden)]
pub struct IssuingAuthorizationFleetFuelPriceDataBuilder {
    gross_amount_decimal: Option<Option<String>>,
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

    impl Deserialize for IssuingAuthorizationFleetFuelPriceData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationFleetFuelPriceData>,
        builder: IssuingAuthorizationFleetFuelPriceDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationFleetFuelPriceData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationFleetFuelPriceDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationFleetFuelPriceDataBuilder {
        type Out = IssuingAuthorizationFleetFuelPriceData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "gross_amount_decimal" => Deserialize::begin(&mut self.gross_amount_decimal),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { gross_amount_decimal: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(gross_amount_decimal),) = (self.gross_amount_decimal.take(),) else {
                return None;
            };
            Some(Self::Out { gross_amount_decimal })
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

    impl ObjectDeser for IssuingAuthorizationFleetFuelPriceData {
        type Builder = IssuingAuthorizationFleetFuelPriceDataBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationFleetFuelPriceData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationFleetFuelPriceDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "gross_amount_decimal" => b.gross_amount_decimal = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
