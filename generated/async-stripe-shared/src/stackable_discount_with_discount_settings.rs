#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct StackableDiscountWithDiscountSettings {
    /// ID of the coupon to create a new discount for.
    pub coupon: Option<stripe_types::Expandable<stripe_shared::Coupon>>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    pub discount: Option<stripe_types::Expandable<stripe_shared::Discount>>,
    /// ID of the promotion code to create a new discount for.
    pub promotion_code: Option<stripe_types::Expandable<stripe_shared::PromotionCode>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for StackableDiscountWithDiscountSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StackableDiscountWithDiscountSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct StackableDiscountWithDiscountSettingsBuilder {
    coupon: Option<stripe_types::Expandable<stripe_shared::Coupon>>,
    discount: Option<stripe_types::Expandable<stripe_shared::Discount>>,
    promotion_code: Option<stripe_types::Expandable<stripe_shared::PromotionCode>>,
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

    impl Deserialize for StackableDiscountWithDiscountSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<StackableDiscountWithDiscountSettings>,
        builder: StackableDiscountWithDiscountSettingsBuilder,
    }

    impl Visitor for Place<StackableDiscountWithDiscountSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: StackableDiscountWithDiscountSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for StackableDiscountWithDiscountSettingsBuilder {
        type Out = StackableDiscountWithDiscountSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon" => stripe_types::miniserde_helpers::begin_nullable(&mut self.coupon),
                "discount" => stripe_types::miniserde_helpers::begin_nullable(&mut self.discount),
                "promotion_code" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.promotion_code)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { coupon: None, discount: None, promotion_code: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { coupon, discount, promotion_code } =
                core::mem::replace(self, Self::deser_default());
            Some(Self::Out { coupon, discount, promotion_code })
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

    impl ObjectDeser for StackableDiscountWithDiscountSettings {
        type Builder = StackableDiscountWithDiscountSettingsBuilder;
    }

    impl FromValueOpt for StackableDiscountWithDiscountSettings {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
