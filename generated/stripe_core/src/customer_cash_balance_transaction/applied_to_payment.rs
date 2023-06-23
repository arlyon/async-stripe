#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AppliedToPayment {
    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were applied to.
    pub payment_intent: stripe_types::Expandable<stripe_core::payment_intent::PaymentIntent>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AppliedToPayment {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
