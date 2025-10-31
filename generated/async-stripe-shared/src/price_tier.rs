#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PriceTier {
    /// Price for the entire tier.
    pub flat_amount: Option<i64>,
    /// Same as `flat_amount`, but contains a decimal value with at most 12 decimal places.
    pub flat_amount_decimal: Option<String>,
    /// Per unit price for units relevant to the tier.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
    /// Up to and including to this quantity will be contained in the tier.
    pub up_to: Option<i64>,
}
#[doc(hidden)]
pub struct PriceTierBuilder {
    flat_amount: Option<Option<i64>>,
    flat_amount_decimal: Option<Option<String>>,
    unit_amount: Option<Option<i64>>,
    unit_amount_decimal: Option<Option<String>>,
    up_to: Option<Option<i64>>,
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

    impl Deserialize for PriceTier {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PriceTier>,
        builder: PriceTierBuilder,
    }

    impl Visitor for Place<PriceTier> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PriceTierBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PriceTierBuilder {
        type Out = PriceTier;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "flat_amount" => Deserialize::begin(&mut self.flat_amount),
                "flat_amount_decimal" => Deserialize::begin(&mut self.flat_amount_decimal),
                "unit_amount" => Deserialize::begin(&mut self.unit_amount),
                "unit_amount_decimal" => Deserialize::begin(&mut self.unit_amount_decimal),
                "up_to" => Deserialize::begin(&mut self.up_to),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                flat_amount: Deserialize::default(),
                flat_amount_decimal: Deserialize::default(),
                unit_amount: Deserialize::default(),
                unit_amount_decimal: Deserialize::default(),
                up_to: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(flat_amount),
                Some(flat_amount_decimal),
                Some(unit_amount),
                Some(unit_amount_decimal),
                Some(up_to),
            ) = (
                self.flat_amount,
                self.flat_amount_decimal.take(),
                self.unit_amount,
                self.unit_amount_decimal.take(),
                self.up_to,
            )
            else {
                return None;
            };
            Some(Self::Out {
                flat_amount,
                flat_amount_decimal,
                unit_amount,
                unit_amount_decimal,
                up_to,
            })
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

    impl ObjectDeser for PriceTier {
        type Builder = PriceTierBuilder;
    }

    impl FromValueOpt for PriceTier {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PriceTierBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "flat_amount" => b.flat_amount = FromValueOpt::from_value(v),
                    "flat_amount_decimal" => b.flat_amount_decimal = FromValueOpt::from_value(v),
                    "unit_amount" => b.unit_amount = FromValueOpt::from_value(v),
                    "unit_amount_decimal" => b.unit_amount_decimal = FromValueOpt::from_value(v),
                    "up_to" => b.up_to = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
