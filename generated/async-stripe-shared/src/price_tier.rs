#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PriceTier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PriceTier").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PriceTierBuilder {
                    flat_amount: Deserialize::default(),
                    flat_amount_decimal: Deserialize::default(),
                    unit_amount: Deserialize::default(),
                    unit_amount_decimal: Deserialize::default(),
                    up_to: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "flat_amount" => Deserialize::begin(&mut self.builder.flat_amount),
                "flat_amount_decimal" => Deserialize::begin(&mut self.builder.flat_amount_decimal),
                "unit_amount" => Deserialize::begin(&mut self.builder.unit_amount),
                "unit_amount_decimal" => Deserialize::begin(&mut self.builder.unit_amount_decimal),
                "up_to" => Deserialize::begin(&mut self.builder.up_to),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(flat_amount),
                Some(flat_amount_decimal),
                Some(unit_amount),
                Some(unit_amount_decimal),
                Some(up_to),
            ) = (
                self.builder.flat_amount,
                self.builder.flat_amount_decimal.take(),
                self.builder.unit_amount,
                self.builder.unit_amount_decimal.take(),
                self.builder.up_to,
            )
            else {
                return Ok(());
            };
            *self.out = Some(PriceTier {
                flat_amount,
                flat_amount_decimal,
                unit_amount,
                unit_amount_decimal,
                up_to,
            });
            Ok(())
        }
    }
};
