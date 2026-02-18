#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions {
pub card: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
pub card_present: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPresentAmountDetailsLineItemPaymentMethodOptions>,
pub klarna: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
pub paypal: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions>,

}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptionsBuilder {
    card: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions>>,
card_present: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPresentAmountDetailsLineItemPaymentMethodOptions>>,
klarna: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions>>,
paypal: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions>>,

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

    impl Deserialize for PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

    struct Builder<'a> {
    out: &'a mut Option<PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions>,
    builder: PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptionsBuilder,
}

    impl Visitor for Place<PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptionsBuilder::deser_default(),
        }))
    }
}

    impl MapBuilder for PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptionsBuilder {
    type Out = PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "card" => Deserialize::begin(&mut self.card),
            "card_present" => Deserialize::begin(&mut self.card_present),
            "klarna" => Deserialize::begin(&mut self.klarna),
            "paypal" => Deserialize::begin(&mut self.paypal),
            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self { card: Deserialize::default(),
card_present: Deserialize::default(),
klarna: Deserialize::default(),
paypal: Deserialize::default(),
 }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(card),
Some(card_present),
Some(klarna),
Some(paypal),
) = (self.card.take(),
self.card_present.take(),
self.klarna.take(),
self.paypal.take(),
) else {
            return None;
        };
        Some(Self::Out { card,card_present,klarna,paypal })
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

    impl ObjectDeser for PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions {
    type Builder = PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptionsBuilder;
}

    impl FromValueOpt for PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptionsBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
            "card" => b.card = FromValueOpt::from_value(v),
            "card_present" => b.card_present = FromValueOpt::from_value(v),
            "klarna" => b.klarna = FromValueOpt::from_value(v),
            "paypal" => b.paypal = FromValueOpt::from_value(v),
                _ => {}
            }
        }
        b.take_out()
    }
}
};
