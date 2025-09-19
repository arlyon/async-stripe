#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionOptionalItem {
    pub adjustable_quantity:
        Option<stripe_shared::PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity>,
    pub price: String,
    pub quantity: u64,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionOptionalItemBuilder {
    adjustable_quantity:
        Option<Option<stripe_shared::PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity>>,
    price: Option<String>,
    quantity: Option<u64>,
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

    impl Deserialize for PaymentPagesCheckoutSessionOptionalItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionOptionalItem>,
        builder: PaymentPagesCheckoutSessionOptionalItemBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionOptionalItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionOptionalItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionOptionalItemBuilder {
        type Out = PaymentPagesCheckoutSessionOptionalItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "adjustable_quantity" => Deserialize::begin(&mut self.adjustable_quantity),
                "price" => Deserialize::begin(&mut self.price),
                "quantity" => Deserialize::begin(&mut self.quantity),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                adjustable_quantity: Deserialize::default(),
                price: Deserialize::default(),
                quantity: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(adjustable_quantity), Some(price), Some(quantity)) =
                (self.adjustable_quantity, self.price.take(), self.quantity)
            else {
                return None;
            };
            Some(Self::Out { adjustable_quantity, price, quantity })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentPagesCheckoutSessionOptionalItem {
        type Builder = PaymentPagesCheckoutSessionOptionalItemBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionOptionalItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionOptionalItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "adjustable_quantity" => b.adjustable_quantity = FromValueOpt::from_value(v),
                    "price" => b.price = FromValueOpt::from_value(v),
                    "quantity" => b.quantity = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
