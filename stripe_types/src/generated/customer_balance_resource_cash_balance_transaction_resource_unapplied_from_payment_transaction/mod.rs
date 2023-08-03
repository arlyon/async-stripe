#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction {
    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were unapplied from.
    pub payment_intent: stripe_types::Expandable<stripe_types::PaymentIntent>,
}
