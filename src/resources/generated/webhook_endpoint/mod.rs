/// You can configure [webhook endpoints](https://stripe.com/docs/webhooks/) via the API to be
/// notified about events that happen in your Stripe account or connected
/// accounts.
///
/// Most users configure webhooks from [the dashboard](https://dashboard.stripe.com/webhooks), which provides a user interface for registering and testing your webhook endpoints.
///
/// Related guide: [Setting up Webhooks](https://stripe.com/docs/webhooks/configure).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct WebhookEndpoint {
    /// The API version events are rendered as for this webhook endpoint.
    pub api_version: Option<String>,
    /// The ID of the associated Connect application.
    pub application: Option<String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// An optional description of what the webhook is used for.
    pub description: Option<String>,
    /// The list of events to enable for this endpoint.
    ///
    /// `['*']` indicates that all events are enabled, except those that require explicit selection.
    pub enabled_events: Vec<String>,
    /// Unique identifier for the object.
    pub id: crate::webhook_endpoint::WebhookEndpointId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: WebhookEndpointObject,
    /// The endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures).
    ///
    /// Only returned at creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The status of the webhook.
    ///
    /// It can be `enabled` or `disabled`.
    pub status: String,
    /// The URL of the webhook endpoint.
    pub url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for WebhookEndpoint {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum WebhookEndpointObject {
    WebhookEndpoint,
}

impl WebhookEndpointObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WebhookEndpoint => "webhook_endpoint",
        }
    }
}

impl AsRef<str> for WebhookEndpointObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WebhookEndpointObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for WebhookEndpoint {
    type Id = crate::webhook_endpoint::WebhookEndpointId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(WebhookEndpointId, "we_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedWebhookEndpoint;
