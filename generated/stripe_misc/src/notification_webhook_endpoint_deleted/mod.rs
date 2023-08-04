#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NotificationWebhookEndpointDeleted {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_misc::notification_webhook_endpoint::WebhookEndpointId,
}
impl stripe_types::Object for NotificationWebhookEndpointDeleted {
    type Id = stripe_misc::notification_webhook_endpoint::WebhookEndpointId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
