#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct UnappliedFromPayment {
    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were unapplied from.
    pub payment_intent: crate::Expandable<crate::payment_intent::PaymentIntent>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for UnappliedFromPayment {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
