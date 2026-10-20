#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionDiscount {
    /// Coupon attached to the Checkout Session.
    pub coupon: Option<stripe_types::Expandable<stripe_shared::Coupon>>,
    /// Promotion code attached to the Checkout Session.
    pub promotion_code: Option<stripe_types::Expandable<stripe_shared::PromotionCode>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionDiscount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionDiscount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionDiscountBuilder {
    coupon: Option<stripe_types::Expandable<stripe_shared::Coupon>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionDiscount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionDiscount>,
        builder: PaymentPagesCheckoutSessionDiscountBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionDiscount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionDiscountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionDiscountBuilder {
        type Out = PaymentPagesCheckoutSessionDiscount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon" => stripe_types::miniserde_helpers::begin_nullable(&mut self.coupon),
                "promotion_code" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.promotion_code)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { coupon: None, promotion_code: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (coupon, promotion_code) = (self.coupon.take(), self.promotion_code.take()) else {
                return None;
            };
            Some(Self::Out { coupon, promotion_code })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionDiscount {
        type Builder = PaymentPagesCheckoutSessionDiscountBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionDiscount {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
