#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ClimateRemovalsProductsPrice {
    /// Fees for one metric ton of carbon removal in the currency's smallest unit.
    pub amount_fees: i64,
    /// Subtotal for one metric ton of carbon removal (excluding fees) in the currency's smallest unit.
    pub amount_subtotal: i64,
    /// Total for one metric ton of carbon removal (including fees) in the currency's smallest unit.
    pub amount_total: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ClimateRemovalsProductsPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClimateRemovalsProductsPrice").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ClimateRemovalsProductsPriceBuilder {
    amount_fees: Option<i64>,
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
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

    impl Deserialize for ClimateRemovalsProductsPrice {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ClimateRemovalsProductsPrice>,
        builder: ClimateRemovalsProductsPriceBuilder,
    }

    impl Visitor for Place<ClimateRemovalsProductsPrice> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ClimateRemovalsProductsPriceBuilder {
                    amount_fees: Deserialize::default(),
                    amount_subtotal: Deserialize::default(),
                    amount_total: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_fees" => Deserialize::begin(&mut self.builder.amount_fees),
                "amount_subtotal" => Deserialize::begin(&mut self.builder.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.builder.amount_total),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount_fees), Some(amount_subtotal), Some(amount_total)) =
                (self.builder.amount_fees, self.builder.amount_subtotal, self.builder.amount_total)
            else {
                return Ok(());
            };
            *self.out =
                Some(ClimateRemovalsProductsPrice { amount_fees, amount_subtotal, amount_total });
            Ok(())
        }
    }
};
