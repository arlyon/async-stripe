#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedWebhookEndpoint {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_misc::webhook_endpoint::WebhookEndpointId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedWebhookEndpointObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedWebhookEndpoint {
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
pub enum DeletedWebhookEndpointObject {
    WebhookEndpoint,
}

impl DeletedWebhookEndpointObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WebhookEndpoint => "webhook_endpoint",
        }
    }
}

impl AsRef<str> for DeletedWebhookEndpointObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedWebhookEndpointObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for DeletedWebhookEndpoint {
    type Id = stripe_misc::webhook_endpoint::WebhookEndpointId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
