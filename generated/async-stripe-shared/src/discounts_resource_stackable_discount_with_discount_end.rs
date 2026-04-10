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
                builder: DiscountsResourceStackableDiscountWithDiscountEndBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DiscountsResourceStackableDiscountWithDiscountEndBuilder {
        type Out = DiscountsResourceStackableDiscountWithDiscountEnd;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon" => Deserialize::begin(&mut self.coupon),
                "discount" => Deserialize::begin(&mut self.discount),
                "promotion_code" => Deserialize::begin(&mut self.promotion_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                coupon: Deserialize::default(),
                discount: Deserialize::default(),
                promotion_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(coupon), Some(discount), Some(promotion_code)) =
                (self.coupon.take(), self.discount.take(), self.promotion_code.take())
            else {
                return None;
            };
            Some(Self::Out { coupon, discount, promotion_code })
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

    impl ObjectDeser for DiscountsResourceStackableDiscountWithDiscountEnd {
        type Builder = DiscountsResourceStackableDiscountWithDiscountEndBuilder;
    }

    impl FromValueOpt for DiscountsResourceStackableDiscountWithDiscountEnd {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DiscountsResourceStackableDiscountWithDiscountEndBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "coupon" => b.coupon = FromValueOpt::from_value(v),
                    "discount" => b.discount = FromValueOpt::from_value(v),
                    "promotion_code" => b.promotion_code = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
