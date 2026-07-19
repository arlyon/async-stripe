#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    /// The aggregated discounts.
    pub discounts: Vec<stripe_shared::LineItemsDiscountAmount>,
    /// The aggregated tax amounts by rate.
    pub taxes: Vec<stripe_shared::LineItemsTaxAmount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder {
    discounts: Option<Vec<stripe_shared::LineItemsDiscountAmount>>,
    taxes: Option<Vec<stripe_shared::LineItemsTaxAmount>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
        builder: PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder {
                    discounts: Deserialize::default(),
                    taxes: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discounts" => Deserialize::begin(&mut self.builder.discounts),
                "taxes" => Deserialize::begin(&mut self.builder.taxes),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(discounts), Some(taxes)) =
                (self.builder.discounts.take(), self.builder.taxes.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown { discounts, taxes });
            Ok(())
        }
    }
};
