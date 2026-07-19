#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DiscountsResourceStackableDiscountWithDiscountEnd {
    /// ID of the coupon to create a new discount for.
    pub coupon: Option<stripe_types::Expandable<stripe_shared::Coupon>>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    pub discount: Option<stripe_types::Expandable<stripe_shared::Discount>>,
    /// ID of the promotion code to create a new discount for.
    pub promotion_code: Option<stripe_types::Expandable<stripe_shared::PromotionCode>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DiscountsResourceStackableDiscountWithDiscountEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DiscountsResourceStackableDiscountWithDiscountEnd").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DiscountsResourceStackableDiscountWithDiscountEndBuilder {
    coupon: Option<Option<stripe_types::Expandable<stripe_shared::Coupon>>>,
    discount: Option<Option<stripe_types::Expandable<stripe_shared::Discount>>>,
    promotion_code: Option<Option<stripe_types::Expandable<stripe_shared::PromotionCode>>>,
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

    impl Deserialize for DiscountsResourceStackableDiscountWithDiscountEnd {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DiscountsResourceStackableDiscountWithDiscountEnd>,
        builder: DiscountsResourceStackableDiscountWithDiscountEndBuilder,
    }

    impl Visitor for Place<DiscountsResourceStackableDiscountWithDiscountEnd> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DiscountsResourceStackableDiscountWithDiscountEndBuilder {
                    coupon: Deserialize::default(),
                    discount: Deserialize::default(),
                    promotion_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon" => Deserialize::begin(&mut self.builder.coupon),
                "discount" => Deserialize::begin(&mut self.builder.discount),
                "promotion_code" => Deserialize::begin(&mut self.builder.promotion_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(coupon), Some(discount), Some(promotion_code)) = (
                self.builder.coupon.take(),
                self.builder.discount.take(),
                self.builder.promotion_code.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(DiscountsResourceStackableDiscountWithDiscountEnd {
                coupon,
                discount,
                promotion_code,
            });
            Ok(())
        }
    }
};
