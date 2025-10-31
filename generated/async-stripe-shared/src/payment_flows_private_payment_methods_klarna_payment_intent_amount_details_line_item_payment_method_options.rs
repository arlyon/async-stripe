#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions
{
    pub image_url: Option<String>,
    pub product_url: Option<String>,
    pub reference: Option<String>,
    pub subscription_reference: Option<String>,
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder
{
    image_url: Option<Option<String>>,
    product_url: Option<Option<String>>,
    reference: Option<Option<String>>,
    subscription_reference: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

    struct Builder<'a> {
    out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
    builder: PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder,
}

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder::deser_default(),
        }))
    }
}

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder {
    type Out = PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "image_url" => Deserialize::begin(&mut self.image_url),
            "product_url" => Deserialize::begin(&mut self.product_url),
            "reference" => Deserialize::begin(&mut self.reference),
            "subscription_reference" => Deserialize::begin(&mut self.subscription_reference),
            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self { image_url: Deserialize::default(),
product_url: Deserialize::default(),
reference: Deserialize::default(),
subscription_reference: Deserialize::default(),
 }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(image_url),
Some(product_url),
Some(reference),
Some(subscription_reference),
) = (self.image_url.take(),
self.product_url.take(),
self.reference.take(),
self.subscription_reference.take(),
) else {
            return None;
        };
        Some(Self::Out { image_url,product_url,reference,subscription_reference })
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

    impl ObjectDeser for PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions {
    type Builder = PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder;
}

    impl FromValueOpt for PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
            "image_url" => b.image_url = FromValueOpt::from_value(v),
            "product_url" => b.product_url = FromValueOpt::from_value(v),
            "reference" => b.reference = FromValueOpt::from_value(v),
            "subscription_reference" => b.subscription_reference = FromValueOpt::from_value(v),
                _ => {}
            }
        }
        b.take_out()
    }
}
};
