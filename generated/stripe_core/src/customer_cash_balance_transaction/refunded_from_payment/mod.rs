#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RefundedFromPayment {
    /// The [Refund](https://stripe.com/docs/api/refunds/object) that moved these funds into the customer's cash balance.
    pub refund: stripe_types::Expandable<stripe_types::refund::Refund>,
}
