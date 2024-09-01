#[derive(Copy, Clone, Debug)]
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
#[doc(hidden)]
pub struct ClimateRemovalsProductsPriceBuilder {
    amount_fees: Option<i64>,
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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
                builder: ClimateRemovalsProductsPriceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ClimateRemovalsProductsPriceBuilder {
        type Out = ClimateRemovalsProductsPrice;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_fees" => Deserialize::begin(&mut self.amount_fees),
                "amount_subtotal" => Deserialize::begin(&mut self.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.amount_total),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_fees: Deserialize::default(),
                amount_subtotal: Deserialize::default(),
                amount_total: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_fees), Some(amount_subtotal), Some(amount_total)) =
                (self.amount_fees, self.amount_subtotal, self.amount_total)
            else {
                return None;
            };
            Some(Self::Out { amount_fees, amount_subtotal, amount_total })
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

    impl ObjectDeser for ClimateRemovalsProductsPrice {
        type Builder = ClimateRemovalsProductsPriceBuilder;
    }

    impl FromValueOpt for ClimateRemovalsProductsPrice {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ClimateRemovalsProductsPriceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_fees" => b.amount_fees = FromValueOpt::from_value(v),
                    "amount_subtotal" => b.amount_subtotal = FromValueOpt::from_value(v),
                    "amount_total" => b.amount_total = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
