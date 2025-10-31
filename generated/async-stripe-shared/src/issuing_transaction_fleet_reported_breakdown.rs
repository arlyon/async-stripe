#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionFleetReportedBreakdown {
    /// Breakdown of fuel portion of the purchase.
    pub fuel: Option<stripe_shared::IssuingTransactionFleetFuelPriceData>,
    /// Breakdown of non-fuel portion of the purchase.
    pub non_fuel: Option<stripe_shared::IssuingTransactionFleetNonFuelPriceData>,
    /// Information about tax included in this transaction.
    pub tax: Option<stripe_shared::IssuingTransactionFleetTaxData>,
}
#[doc(hidden)]
pub struct IssuingTransactionFleetReportedBreakdownBuilder {
    fuel: Option<Option<stripe_shared::IssuingTransactionFleetFuelPriceData>>,
    non_fuel: Option<Option<stripe_shared::IssuingTransactionFleetNonFuelPriceData>>,
    tax: Option<Option<stripe_shared::IssuingTransactionFleetTaxData>>,
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

    impl Deserialize for IssuingTransactionFleetReportedBreakdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFleetReportedBreakdown>,
        builder: IssuingTransactionFleetReportedBreakdownBuilder,
    }

    impl Visitor for Place<IssuingTransactionFleetReportedBreakdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionFleetReportedBreakdownBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionFleetReportedBreakdownBuilder {
        type Out = IssuingTransactionFleetReportedBreakdown;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fuel" => Deserialize::begin(&mut self.fuel),
                "non_fuel" => Deserialize::begin(&mut self.non_fuel),
                "tax" => Deserialize::begin(&mut self.tax),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                fuel: Deserialize::default(),
                non_fuel: Deserialize::default(),
                tax: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(fuel), Some(non_fuel), Some(tax)) =
                (self.fuel.take(), self.non_fuel.take(), self.tax.take())
            else {
                return None;
            };
            Some(Self::Out { fuel, non_fuel, tax })
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

    impl ObjectDeser for IssuingTransactionFleetReportedBreakdown {
        type Builder = IssuingTransactionFleetReportedBreakdownBuilder;
    }

    impl FromValueOpt for IssuingTransactionFleetReportedBreakdown {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionFleetReportedBreakdownBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fuel" => b.fuel = FromValueOpt::from_value(v),
                    "non_fuel" => b.non_fuel = FromValueOpt::from_value(v),
                    "tax" => b.tax = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
