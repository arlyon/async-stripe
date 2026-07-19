#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Level3 {
    pub customer_reference: Option<String>,
    pub line_items: Vec<stripe_shared::Level3LineItems>,
    pub merchant_reference: String,
    pub shipping_address_zip: Option<String>,
    pub shipping_amount: Option<i64>,
    pub shipping_from_zip: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Level3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Level3").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct Level3Builder {
    customer_reference: Option<Option<String>>,
    line_items: Option<Vec<stripe_shared::Level3LineItems>>,
    merchant_reference: Option<String>,
    shipping_address_zip: Option<Option<String>>,
    shipping_amount: Option<Option<i64>>,
    shipping_from_zip: Option<Option<String>>,
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

    impl Deserialize for Level3 {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Level3>,
        builder: Level3Builder,
    }

    impl Visitor for Place<Level3> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: Level3Builder {
                    customer_reference: Deserialize::default(),
                    line_items: Deserialize::default(),
                    merchant_reference: Deserialize::default(),
                    shipping_address_zip: Deserialize::default(),
                    shipping_amount: Deserialize::default(),
                    shipping_from_zip: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_reference" => Deserialize::begin(&mut self.builder.customer_reference),
                "line_items" => Deserialize::begin(&mut self.builder.line_items),
                "merchant_reference" => Deserialize::begin(&mut self.builder.merchant_reference),
                "shipping_address_zip" => {
                    Deserialize::begin(&mut self.builder.shipping_address_zip)
                }
                "shipping_amount" => Deserialize::begin(&mut self.builder.shipping_amount),
                "shipping_from_zip" => Deserialize::begin(&mut self.builder.shipping_from_zip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(customer_reference),
                Some(line_items),
                Some(merchant_reference),
                Some(shipping_address_zip),
                Some(shipping_amount),
                Some(shipping_from_zip),
            ) = (
                self.builder.customer_reference.take(),
                self.builder.line_items.take(),
                self.builder.merchant_reference.take(),
                self.builder.shipping_address_zip.take(),
                self.builder.shipping_amount,
                self.builder.shipping_from_zip.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(Level3 {
                customer_reference,
                line_items,
                merchant_reference,
                shipping_address_zip,
                shipping_amount,
                shipping_from_zip,
            });
            Ok(())
        }
    }
};
