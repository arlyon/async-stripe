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
    error: Option<stripe_shared::PaymentFlowsAmountDetailsResourceError>,
    line_items:
        Option<Option<stripe_types::List<stripe_shared::PaymentIntentAmountDetailsLineItem>>>,
    shipping: Option<stripe_shared::PaymentFlowsAmountDetailsResourceShipping>,
    tax: Option<stripe_shared::PaymentFlowsAmountDetailsResourceTax>,
    tip: Option<stripe_shared::PaymentFlowsAmountDetailsClientResourceTip>,
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
                "error" => stripe_types::miniserde_helpers::begin_nullable(&mut self.error),
                "line_items" => Deserialize::begin(&mut self.line_items),
                "shipping" => stripe_types::miniserde_helpers::begin_nullable(&mut self.shipping),
                "tax" => stripe_types::miniserde_helpers::begin_nullable(&mut self.tax),
                "tip" => stripe_types::miniserde_helpers::begin_nullable(&mut self.tip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                discount_amount: Some(None),
                error: None,
                line_items: Some(None),
                shipping: None,
                tax: None,
                tip: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    discount_amount: Some(discount_amount),
                    error,
                    line_items: Some(line_items),
                    shipping,
                    tax,
                    tip,
                } => Some(Self::Out { discount_amount, error, line_items, shipping, tax, tip }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for PaymentFlowsAmountDetails {
        type Builder = PaymentFlowsAmountDetailsBuilder;
    }

    impl FromValueOpt for PaymentFlowsAmountDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
