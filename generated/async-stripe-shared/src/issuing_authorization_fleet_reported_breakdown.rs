#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationFleetReportedBreakdown {
    /// Breakdown of fuel portion of the purchase.
    pub fuel: Option<stripe_shared::IssuingAuthorizationFleetFuelPriceData>,
    /// Breakdown of non-fuel portion of the purchase.
    pub non_fuel: Option<stripe_shared::IssuingAuthorizationFleetNonFuelPriceData>,
    /// Information about tax included in this transaction.
    pub tax: Option<stripe_shared::IssuingAuthorizationFleetTaxData>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFleetReportedBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationFleetReportedBreakdown").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationFleetReportedBreakdownBuilder {
    fuel: Option<Option<stripe_shared::IssuingAuthorizationFleetFuelPriceData>>,
    non_fuel: Option<Option<stripe_shared::IssuingAuthorizationFleetNonFuelPriceData>>,
    tax: Option<Option<stripe_shared::IssuingAuthorizationFleetTaxData>>,
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

    impl Deserialize for IssuingAuthorizationFleetReportedBreakdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationFleetReportedBreakdown>,
        builder: IssuingAuthorizationFleetReportedBreakdownBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationFleetReportedBreakdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationFleetReportedBreakdownBuilder {
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
            *self.out = Some(IssuingAuthorizationFleetReportedBreakdown { fuel, non_fuel, tax });
            Ok(())
        }
    }
};
