#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionFleetReportedBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionFleetReportedBreakdown").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingTransactionFleetReportedBreakdownBuilder {
                    fuel: Deserialize::default(),
                    non_fuel: Deserialize::default(),
                    tax: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fuel" => Deserialize::begin(&mut self.builder.fuel),
                "non_fuel" => Deserialize::begin(&mut self.builder.non_fuel),
                "tax" => Deserialize::begin(&mut self.builder.tax),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(fuel), Some(non_fuel), Some(tax)) =
                (self.builder.fuel.take(), self.builder.non_fuel.take(), self.builder.tax.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionFleetReportedBreakdown { fuel, non_fuel, tax });
            Ok(())
        }
    }
};
