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
    fuel: Option<stripe_shared::IssuingAuthorizationFleetFuelPriceData>,
    non_fuel: Option<stripe_shared::IssuingAuthorizationFleetNonFuelPriceData>,
    tax: Option<stripe_shared::IssuingAuthorizationFleetTaxData>,
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
                builder: IssuingAuthorizationFleetReportedBreakdownBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationFleetReportedBreakdownBuilder {
        type Out = IssuingAuthorizationFleetReportedBreakdown;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fuel" => stripe_types::miniserde_helpers::begin_nullable(&mut self.fuel),
                "non_fuel" => stripe_types::miniserde_helpers::begin_nullable(&mut self.non_fuel),
                "tax" => stripe_types::miniserde_helpers::begin_nullable(&mut self.tax),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { fuel: None, non_fuel: None, tax: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { fuel, non_fuel, tax } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out { fuel, non_fuel, tax })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for IssuingAuthorizationFleetReportedBreakdown {
        type Builder = IssuingAuthorizationFleetReportedBreakdownBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationFleetReportedBreakdown {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
