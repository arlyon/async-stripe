#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LineItemsDiscountAmount {
    /// The amount discounted.
    pub amount: i64,
    pub discount: stripe_shared::Discount,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LineItemsDiscountAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LineItemsDiscountAmount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct LineItemsDiscountAmountBuilder {
    amount: Option<i64>,
    discount: Option<stripe_shared::Discount>,
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

    impl Deserialize for LineItemsDiscountAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LineItemsDiscountAmount>,
        builder: LineItemsDiscountAmountBuilder,
    }

    impl Visitor for Place<LineItemsDiscountAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LineItemsDiscountAmountBuilder {
                    amount: Deserialize::default(),
                    discount: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "discount" => Deserialize::begin(&mut self.builder.discount),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(discount)) =
                (self.builder.amount, self.builder.discount.take())
            else {
                return Ok(());
            };
            *self.out = Some(LineItemsDiscountAmount { amount, discount });
            Ok(())
        }
    }
};
