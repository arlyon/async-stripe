#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DiscountsResourceDiscountAmount {
    /// The amount, in cents (or local equivalent), of the discount.
    pub amount: i64,
    /// The discount that was applied to get this discount amount.
    pub discount: stripe_types::Expandable<stripe_shared::Discount>,
}
#[doc(hidden)]
pub struct DiscountsResourceDiscountAmountBuilder {
    amount: Option<i64>,
    discount: Option<stripe_types::Expandable<stripe_shared::Discount>>,
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

    impl Deserialize for DiscountsResourceDiscountAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DiscountsResourceDiscountAmount>,
        builder: DiscountsResourceDiscountAmountBuilder,
    }

    impl Visitor for Place<DiscountsResourceDiscountAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DiscountsResourceDiscountAmountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DiscountsResourceDiscountAmountBuilder {
        type Out = DiscountsResourceDiscountAmount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "discount" => Deserialize::begin(&mut self.discount),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), discount: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(discount)) = (self.amount, self.discount.take()) else {
                return None;
            };
            Some(Self::Out { amount, discount })
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

    impl ObjectDeser for DiscountsResourceDiscountAmount {
        type Builder = DiscountsResourceDiscountAmountBuilder;
    }

    impl FromValueOpt for DiscountsResourceDiscountAmount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DiscountsResourceDiscountAmountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "discount" => b.discount = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
