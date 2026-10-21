#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceOptionalItem {
    pub adjustable_quantity:
        Option<stripe_shared::PaymentLinksResourceOptionalItemAdjustableQuantity>,
    pub price: String,
    pub quantity: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceOptionalItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceOptionalItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceOptionalItemBuilder {
    adjustable_quantity: Option<stripe_shared::PaymentLinksResourceOptionalItemAdjustableQuantity>,
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

    impl Deserialize for PaymentLinksResourceOptionalItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceOptionalItem>,
        builder: PaymentLinksResourceOptionalItemBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceOptionalItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceOptionalItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceOptionalItemBuilder {
        type Out = PaymentLinksResourceOptionalItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "adjustable_quantity" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.adjustable_quantity)
                }
                "price" => Deserialize::begin(&mut self.price),
                "quantity" => Deserialize::begin(&mut self.quantity),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { adjustable_quantity: None, price: None, quantity: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { adjustable_quantity, price: Some(price), quantity: Some(quantity) } => {
                    Some(Self::Out { adjustable_quantity, price, quantity })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for PaymentLinksResourceOptionalItem {
        type Builder = PaymentLinksResourceOptionalItemBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceOptionalItem {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
