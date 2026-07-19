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
    coupon: Option<Option<stripe_types::Expandable<stripe_shared::Coupon>>>,
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
                builder: PaymentPagesCheckoutSessionDiscountBuilder {
                    coupon: Deserialize::default(),
                    promotion_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon" => Deserialize::begin(&mut self.builder.coupon),
                "promotion_code" => Deserialize::begin(&mut self.builder.promotion_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(coupon), Some(promotion_code)) =
                (self.builder.coupon.take(), self.builder.promotion_code.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionDiscount { coupon, promotion_code });
            Ok(())
        }
    }
};
