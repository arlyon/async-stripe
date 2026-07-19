#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetails {
    /// The total discount applied on the transaction represented in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    /// An integer greater than 0.
    ///
    /// This field is mutually exclusive with the `amount_details[line_items][#][discount_amount]` field.
    pub discount_amount: Option<i64>,
    pub error: Option<stripe_shared::PaymentFlowsAmountDetailsResourceError>,
    /// A list of line items, each containing information about a product in the PaymentIntent.
    /// There is a maximum of 200 line items.
    pub line_items: Option<stripe_types::List<stripe_shared::PaymentIntentAmountDetailsLineItem>>,
    pub shipping: Option<stripe_shared::PaymentFlowsAmountDetailsResourceShipping>,
    pub tax: Option<stripe_shared::PaymentFlowsAmountDetailsResourceTax>,
    pub tip: Option<stripe_shared::PaymentFlowsAmountDetailsClientResourceTip>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsAmountDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsAmountDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsBuilder {
    discount_amount: Option<Option<i64>>,
    error: Option<Option<stripe_shared::PaymentFlowsAmountDetailsResourceError>>,
    line_items:
        Option<Option<stripe_types::List<stripe_shared::PaymentIntentAmountDetailsLineItem>>>,
    shipping: Option<Option<stripe_shared::PaymentFlowsAmountDetailsResourceShipping>>,
    tax: Option<Option<stripe_shared::PaymentFlowsAmountDetailsResourceTax>>,
    tip: Option<Option<stripe_shared::PaymentFlowsAmountDetailsClientResourceTip>>,
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

    impl Deserialize for PaymentFlowsAmountDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAmountDetails>,
        builder: PaymentFlowsAmountDetailsBuilder,
    }

    impl Visitor for Place<PaymentFlowsAmountDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsAmountDetailsBuilder {
                    discount_amount: Deserialize::default(),
                    error: Deserialize::default(),
                    line_items: Deserialize::default(),
                    shipping: Deserialize::default(),
                    tax: Deserialize::default(),
                    tip: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discount_amount" => Deserialize::begin(&mut self.builder.discount_amount),
                "error" => Deserialize::begin(&mut self.builder.error),
                "line_items" => Deserialize::begin(&mut self.builder.line_items),
                "shipping" => Deserialize::begin(&mut self.builder.shipping),
                "tax" => Deserialize::begin(&mut self.builder.tax),
                "tip" => Deserialize::begin(&mut self.builder.tip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(discount_amount),
                Some(error),
                Some(line_items),
                Some(shipping),
                Some(tax),
                Some(tip),
            ) = (
                self.builder.discount_amount,
                self.builder.error.take(),
                self.builder.line_items.take(),
                self.builder.shipping.take(),
                self.builder.tax,
                self.builder.tip,
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsAmountDetails {
                discount_amount,
                error,
                line_items,
                shipping,
                tax,
                tip,
            });
            Ok(())
        }
    }
};
