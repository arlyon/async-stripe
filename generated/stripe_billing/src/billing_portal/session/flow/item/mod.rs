#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Item {
    /// The ID of the [subscription item](https://stripe.com/docs/api/subscriptions/object#subscription_object-items-data-id) to be updated.
pub id: Option<stripe_billing::billing_portal::session::flow::item::PortalFlowsSubscriptionUpdateConfirmItemId>,
    /// The price the customer should subscribe to through this flow.
    ///
    /// The price must also be included in the configuration's [`features.subscription_update.products`](docs/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
pub price: Option<String>,
    /// [Quantity](https://stripe.com/docs/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
#[serde(skip_serializing_if = "Option::is_none")]
pub quantity: Option<u64>,

}
impl stripe_types::Object for Item {
    type Id = Option<stripe_billing::billing_portal::session::flow::item::PortalFlowsSubscriptionUpdateConfirmItemId>;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PortalFlowsSubscriptionUpdateConfirmItemId);
