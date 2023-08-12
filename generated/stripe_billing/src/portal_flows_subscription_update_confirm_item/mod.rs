#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PortalFlowsSubscriptionUpdateConfirmItem {
    /// The ID of the [subscription item](https://stripe.com/docs/api/subscriptions/object#subscription_object-items-data-id) to be updated.
pub id: Option<stripe_billing::portal_flows_subscription_update_confirm_item::PortalFlowsSubscriptionUpdateConfirmItemId>,
    /// The price the customer should subscribe to through this flow.
    ///
    /// The price must also be included in the configuration's [`features.subscription_update.products`](docs/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
pub price: Option<String>,
    /// [Quantity](https://stripe.com/docs/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
#[serde(skip_serializing_if = "Option::is_none")]
pub quantity: Option<u64>,

}
impl stripe_types::Object for PortalFlowsSubscriptionUpdateConfirmItem {
    type Id = Option<stripe_billing::portal_flows_subscription_update_confirm_item::PortalFlowsSubscriptionUpdateConfirmItemId>;
    fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|i| i.as_str())
    }
}
stripe_types::def_id!(PortalFlowsSubscriptionUpdateConfirmItemId);
