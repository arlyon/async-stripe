#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppliedToPayment {
    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were applied to.
    pub payment_intent: stripe_types::Expandable<stripe_types::payment_intent::PaymentIntent>,
}
