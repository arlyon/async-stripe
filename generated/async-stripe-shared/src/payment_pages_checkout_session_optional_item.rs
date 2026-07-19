#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionOptionalItem {
    pub adjustable_quantity:
        Option<stripe_shared::PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity>,
    pub price: String,
    pub quantity: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionOptionalItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionOptionalItem").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentPagesCheckoutSessionOptionalItemBuilder {
                    adjustable_quantity: Deserialize::default(),
                    price: Deserialize::default(),
                    quantity: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "adjustable_quantity" => Deserialize::begin(&mut self.builder.adjustable_quantity),
                "price" => Deserialize::begin(&mut self.builder.price),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(adjustable_quantity), Some(price), Some(quantity)) = (
                self.builder.adjustable_quantity,
                self.builder.price.take(),
                self.builder.quantity,
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionOptionalItem {
                adjustable_quantity,
                price,
                quantity,
            });
            Ok(())
        }
    }
};
