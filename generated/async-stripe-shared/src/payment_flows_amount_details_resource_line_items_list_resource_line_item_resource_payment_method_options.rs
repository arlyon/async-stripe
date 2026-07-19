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
    card: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions>>,
card_present: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardPresentAmountDetailsLineItemPaymentMethodOptions>>,
klarna: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions>>,
paypal: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions>>,

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
            builder: PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptionsBuilder { card: Deserialize::default(),
card_present: Deserialize::default(),
klarna: Deserialize::default(),
paypal: Deserialize::default(),
 },
        }))
    }
}

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.builder.card),
                "card_present" => Deserialize::begin(&mut self.builder.card_present),
                "klarna" => Deserialize::begin(&mut self.builder.klarna),
                "paypal" => Deserialize::begin(&mut self.builder.paypal),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(card), Some(card_present), Some(klarna), Some(paypal)) = (
                self.builder.card.take(),
                self.builder.card_present.take(),
                self.builder.klarna.take(),
                self.builder.paypal.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions { card,card_present,klarna,paypal });
            Ok(())
        }
    }
};
