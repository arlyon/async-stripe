#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionFuelData {
    /// The type of fuel that was purchased.
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    /// The units for `volume_decimal`. One of `liter`, `us_gallon`, or `other`.
    pub unit: String,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    pub unit_cost_decimal: String,
    /// The volume of the fuel that was pumped, represented as a decimal string with at most 12 decimal places.
    pub volume_decimal: Option<String>,
}
#[doc(hidden)]
pub struct IssuingTransactionFuelDataBuilder {
    type_: Option<String>,
    unit: Option<String>,
    unit_cost_decimal: Option<String>,
    volume_decimal: Option<Option<String>>,
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

    impl Deserialize for IssuingTransactionFuelData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFuelData>,
        builder: IssuingTransactionFuelDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionFuelData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionFuelDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionFuelDataBuilder {
        type Out = IssuingTransactionFuelData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.type_),
                "unit" => Deserialize::begin(&mut self.unit),
                "unit_cost_decimal" => Deserialize::begin(&mut self.unit_cost_decimal),
                "volume_decimal" => Deserialize::begin(&mut self.volume_decimal),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                type_: Deserialize::default(),
                unit: Deserialize::default(),
                unit_cost_decimal: Deserialize::default(),
                volume_decimal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(type_), Some(unit), Some(unit_cost_decimal), Some(volume_decimal)) = (
                self.type_.take(),
                self.unit.take(),
                self.unit_cost_decimal.take(),
                self.volume_decimal.take(),
            ) else {
                return None;
            };
            Some(Self::Out { type_, unit, unit_cost_decimal, volume_decimal })
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

    impl ObjectDeser for IssuingTransactionFuelData {
        type Builder = IssuingTransactionFuelDataBuilder;
    }

    impl FromValueOpt for IssuingTransactionFuelData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionFuelDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "unit" => b.unit = FromValueOpt::from_value(v),
                    "unit_cost_decimal" => b.unit_cost_decimal = FromValueOpt::from_value(v),
                    "volume_decimal" => b.volume_decimal = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
