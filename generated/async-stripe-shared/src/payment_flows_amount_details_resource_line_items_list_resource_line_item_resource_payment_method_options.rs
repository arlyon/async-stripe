#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions {
pub card: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
pub card_present: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPresentAmountDetailsLineItemPaymentMethodOptions>,
pub klarna: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
pub paypal: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptionsBuilder {
    card: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
card_present: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPresentAmountDetailsLineItemPaymentMethodOptions>,
klarna: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
paypal: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions>,

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
            "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
            "card_present" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card_present),
            "klarna" => stripe_types::miniserde_helpers::begin_nullable(&mut self.klarna),
            "paypal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.paypal),
            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self { card: None,
card_present: None,
klarna: None,
paypal: None,
 }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let Self { card,
card_present,
klarna,
paypal,
 } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out { card,card_present,klarna,paypal })
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

    impl ObjectDeser for PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions {
    type Builder = PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptionsBuilder;
}

    impl FromValueOpt for PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions {
    fn from_value(v: Value) -> Option<Self> {
        stripe_types::miniserde_helpers::from_value_via_deserialize(v)
    }
}
};
