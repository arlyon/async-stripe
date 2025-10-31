#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions
{
    pub commodity_code: Option<String>,
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder
{
    commodity_code: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

    struct Builder<'a> {
    out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
    builder: PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder,
}

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder::deser_default(),
        }))
    }
}

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder {
    type Out = PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "commodity_code" => Deserialize::begin(&mut self.commodity_code),
            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self { commodity_code: Deserialize::default(),
 }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(commodity_code),
) = (self.commodity_code.take(),
) else {
            return None;
        };
        Some(Self::Out { commodity_code })
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

    impl ObjectDeser for PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions {
    type Builder = PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder;
}

    impl FromValueOpt for PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptionsBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
            "commodity_code" => b.commodity_code = FromValueOpt::from_value(v),
                _ => {}
            }
        }
        b.take_out()
    }
}
};
