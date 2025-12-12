#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetails {
    /// The total discount applied on the transaction represented in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    /// An integer greater than 0.
    ///
    /// This field is mutually exclusive with the `amount_details[line_items][#][discount_amount]` field.
    pub discount_amount: Option<i64>,
    /// A list of line items, each containing information about a product in the PaymentIntent.
    /// There is a maximum of 200 line items.
    pub line_items: Option<stripe_types::List<stripe_shared::PaymentIntentAmountDetailsLineItem>>,
    pub shipping: Option<stripe_shared::PaymentFlowsAmountDetailsResourceShipping>,
    pub tax: Option<stripe_shared::PaymentFlowsAmountDetailsResourceTax>,
    pub tip: Option<stripe_shared::PaymentFlowsAmountDetailsClientResourceTip>,
}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsBuilder {
    discount_amount: Option<Option<i64>>,
    line_items:
        Option<Option<stripe_types::List<stripe_shared::PaymentIntentAmountDetailsLineItem>>>,
    shipping: Option<Option<stripe_shared::PaymentFlowsAmountDetailsResourceShipping>>,
    tax: Option<Option<stripe_shared::PaymentFlowsAmountDetailsResourceTax>>,
    tip: Option<Option<stripe_shared::PaymentFlowsAmountDetailsClientResourceTip>>,
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
                builder: PaymentFlowsAmountDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsAmountDetailsBuilder {
        type Out = PaymentFlowsAmountDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discount_amount" => Deserialize::begin(&mut self.discount_amount),
                "line_items" => Deserialize::begin(&mut self.line_items),
                "shipping" => Deserialize::begin(&mut self.shipping),
                "tax" => Deserialize::begin(&mut self.tax),
                "tip" => Deserialize::begin(&mut self.tip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                discount_amount: Deserialize::default(),
                line_items: Deserialize::default(),
                shipping: Deserialize::default(),
                tax: Deserialize::default(),
                tip: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(discount_amount), Some(line_items), Some(shipping), Some(tax), Some(tip)) = (
                self.discount_amount,
                self.line_items.take(),
                self.shipping.take(),
                self.tax,
                self.tip,
            ) else {
                return None;
            };
            Some(Self::Out { discount_amount, line_items, shipping, tax, tip })
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

    impl ObjectDeser for PaymentFlowsAmountDetails {
        type Builder = PaymentFlowsAmountDetailsBuilder;
    }

    impl FromValueOpt for PaymentFlowsAmountDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsAmountDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "discount_amount" => b.discount_amount = FromValueOpt::from_value(v),
                    "line_items" => b.line_items = FromValueOpt::from_value(v),
                    "shipping" => b.shipping = FromValueOpt::from_value(v),
                    "tax" => b.tax = FromValueOpt::from_value(v),
                    "tip" => b.tip = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
