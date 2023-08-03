#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalFlowsFlowSubscriptionUpdateConfirm {
    /// The coupon or promotion code to apply to this subscription update.
    ///
    /// Currently, only up to one may be specified.
    pub discounts: Option<Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmDiscount>>,
    /// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
    ///
    /// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
    pub items: Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItem>,
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
