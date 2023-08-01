#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionUpdateConfirm {
    /// The coupon or promotion code to apply to this subscription update.
    ///
    /// Currently, only up to one may be specified.
    pub discounts: Option<Vec<stripe_billing::billing_portal::session::flow::discount::Discount>>,
    /// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
    ///
    /// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
    pub items: Vec<stripe_billing::billing_portal::session::flow::item::Item>,
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
