#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Level3LineItems {
    pub discount_amount: Option<i64>,
    pub product_code: String,
    pub product_description: String,
    pub quantity: Option<u64>,
    pub tax_amount: Option<i64>,
    pub unit_cost: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Level3LineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Level3LineItems").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct Level3LineItemsBuilder {
    discount_amount: Option<Option<i64>>,
    product_code: Option<String>,
    product_description: Option<String>,
    quantity: Option<Option<u64>>,
    tax_amount: Option<Option<i64>>,
    unit_cost: Option<Option<i64>>,
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

    impl Deserialize for Level3LineItems {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Level3LineItems>,
        builder: Level3LineItemsBuilder,
    }

    impl Visitor for Place<Level3LineItems> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: Level3LineItemsBuilder {
                    discount_amount: Deserialize::default(),
                    product_code: Deserialize::default(),
                    product_description: Deserialize::default(),
                    quantity: Deserialize::default(),
                    tax_amount: Deserialize::default(),
                    unit_cost: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discount_amount" => Deserialize::begin(&mut self.builder.discount_amount),
                "product_code" => Deserialize::begin(&mut self.builder.product_code),
                "product_description" => Deserialize::begin(&mut self.builder.product_description),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "tax_amount" => Deserialize::begin(&mut self.builder.tax_amount),
                "unit_cost" => Deserialize::begin(&mut self.builder.unit_cost),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(discount_amount),
                Some(product_code),
                Some(product_description),
                Some(quantity),
                Some(tax_amount),
                Some(unit_cost),
            ) = (
                self.builder.discount_amount,
                self.builder.product_code.take(),
                self.builder.product_description.take(),
                self.builder.quantity,
                self.builder.tax_amount,
                self.builder.unit_cost,
            )
            else {
                return Ok(());
            };
            *self.out = Some(Level3LineItems {
                discount_amount,
                product_code,
                product_description,
                quantity,
                tax_amount,
                unit_cost,
            });
            Ok(())
        }
    }
};
