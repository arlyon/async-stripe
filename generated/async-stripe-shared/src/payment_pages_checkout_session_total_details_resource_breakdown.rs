#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    /// The aggregated discounts.
    pub discounts: Vec<stripe_shared::LineItemsDiscountAmount>,
    /// The aggregated tax amounts by rate.
    pub taxes: Vec<stripe_shared::LineItemsTaxAmount>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder {
    discounts: Option<Vec<stripe_shared::LineItemsDiscountAmount>>,
    taxes: Option<Vec<stripe_shared::LineItemsTaxAmount>>,
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
                builder:
                    PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder {
        type Out = PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discounts" => Deserialize::begin(&mut self.discounts),
                "taxes" => Deserialize::begin(&mut self.taxes),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { discounts: Deserialize::default(), taxes: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(discounts), Some(taxes)) = (self.discounts.take(), self.taxes.take()) else {
                return None;
            };
            Some(Self::Out { discounts, taxes })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
        type Builder = PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "discounts" => b.discounts = FromValueOpt::from_value(v),
                    "taxes" => b.taxes = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
