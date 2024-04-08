/// You can configure [webhook endpoints](https://stripe.com/docs/webhooks/) via the API to be
/// notified about events that happen in your Stripe account or connected
/// accounts.
///
/// Most users configure webhooks from [the dashboard](https://dashboard.stripe.com/webhooks), which provides a user interface for registering and testing your webhook endpoints.
///
/// Related guide: [Setting up webhooks](https://stripe.com/docs/webhooks/configure)
///
/// For more details see <<https://stripe.com/docs/api/webhook_endpoints/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WebhookEndpoint {
    /// The API version events are rendered as for this webhook endpoint.
    pub api_version: Option<String>,
    /// The ID of the associated Connect application.
    pub application: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// An optional description of what the webhook is used for.
    pub description: Option<String>,
    /// The list of events to enable for this endpoint.
    /// `['*']` indicates that all events are enabled, except those that require explicit selection.
    pub enabled_events: Vec<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::WebhookEndpointId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures).
    /// Only returned at creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The status of the webhook. It can be `enabled` or `disabled`.
    pub status: String,
    /// The URL of the webhook endpoint.
    pub url: String,
}
impl stripe_types::Object for WebhookEndpoint {
    type Id = stripe_misc::WebhookEndpointId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(WebhookEndpointId);
