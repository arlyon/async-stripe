#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationFleetNonFuelPriceData {
    /// Gross non-fuel amount that should equal the sum of the line items, inclusive of taxes.
    pub gross_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFleetNonFuelPriceData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationFleetNonFuelPriceData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationFleetNonFuelPriceDataBuilder {
    gross_amount_decimal: Option<Option<String>>,
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

    impl Deserialize for IssuingAuthorizationFleetNonFuelPriceData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationFleetNonFuelPriceData>,
        builder: IssuingAuthorizationFleetNonFuelPriceDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationFleetNonFuelPriceData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationFleetNonFuelPriceDataBuilder {
                    gross_amount_decimal: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "gross_amount_decimal" => {
                    Deserialize::begin(&mut self.builder.gross_amount_decimal)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(gross_amount_decimal),) = (self.builder.gross_amount_decimal.take(),) else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorizationFleetNonFuelPriceData { gross_amount_decimal });
            Ok(())
        }
    }
};
