#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedWebhookEndpoint {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_misc::WebhookEndpointId,
}
impl stripe_types::Object for DeletedWebhookEndpoint {
    type Id = stripe_misc::WebhookEndpointId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
