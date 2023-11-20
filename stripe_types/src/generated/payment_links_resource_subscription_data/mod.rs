#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    pub metadata: std::collections::HashMap<String, String>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,
}
