#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionTotalDetails {
    /// This is the sum of all the discounts.
    pub amount_discount: i64,
    /// This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,
    /// This is the sum of all the tax amounts.
    pub amount_tax: i64,
    pub breakdown: Option<stripe_shared::PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionTotalDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionTotalDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionTotalDetailsBuilder {
    amount_discount: Option<i64>,
    amount_shipping: Option<Option<i64>>,
    amount_tax: Option<i64>,
    breakdown:
        Option<Option<stripe_shared::PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionTotalDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionTotalDetails>,
        builder: PaymentPagesCheckoutSessionTotalDetailsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionTotalDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionTotalDetailsBuilder {
                    amount_discount: Deserialize::default(),
                    amount_shipping: Deserialize::default(),
                    amount_tax: Deserialize::default(),
                    breakdown: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_discount" => Deserialize::begin(&mut self.builder.amount_discount),
                "amount_shipping" => Deserialize::begin(&mut self.builder.amount_shipping),
                "amount_tax" => Deserialize::begin(&mut self.builder.amount_tax),
                "breakdown" => Deserialize::begin(&mut self.builder.breakdown),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount_discount), Some(amount_shipping), Some(amount_tax), Some(breakdown)) = (
                self.builder.amount_discount,
                self.builder.amount_shipping,
                self.builder.amount_tax,
                self.builder.breakdown.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionTotalDetails {
                amount_discount,
                amount_shipping,
                amount_tax,
                breakdown,
            });
            Ok(())
        }
    }
};
