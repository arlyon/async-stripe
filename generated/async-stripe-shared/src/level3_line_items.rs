#[derive(Clone, Debug)]
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
                builder: Level3LineItemsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for Level3LineItemsBuilder {
        type Out = Level3LineItems;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discount_amount" => Deserialize::begin(&mut self.discount_amount),
                "product_code" => Deserialize::begin(&mut self.product_code),
                "product_description" => Deserialize::begin(&mut self.product_description),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "tax_amount" => Deserialize::begin(&mut self.tax_amount),
                "unit_cost" => Deserialize::begin(&mut self.unit_cost),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                discount_amount: Deserialize::default(),
                product_code: Deserialize::default(),
                product_description: Deserialize::default(),
                quantity: Deserialize::default(),
                tax_amount: Deserialize::default(),
                unit_cost: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(discount_amount),
                Some(product_code),
                Some(product_description),
                Some(quantity),
                Some(tax_amount),
                Some(unit_cost),
            ) = (
                self.discount_amount,
                self.product_code.take(),
                self.product_description.take(),
                self.quantity,
                self.tax_amount,
                self.unit_cost,
            )
            else {
                return None;
            };
            Some(Self::Out {
                discount_amount,
                product_code,
                product_description,
                quantity,
                tax_amount,
                unit_cost,
            })
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

    impl ObjectDeser for Level3LineItems {
        type Builder = Level3LineItemsBuilder;
    }

    impl FromValueOpt for Level3LineItems {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = Level3LineItemsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "discount_amount" => b.discount_amount = FromValueOpt::from_value(v),
                    "product_code" => b.product_code = FromValueOpt::from_value(v),
                    "product_description" => b.product_description = FromValueOpt::from_value(v),
                    "quantity" => b.quantity = FromValueOpt::from_value(v),
                    "tax_amount" => b.tax_amount = FromValueOpt::from_value(v),
                    "unit_cost" => b.unit_cost = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
