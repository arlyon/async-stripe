#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NotificationWebhookEndpointDeleted {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_misc::notification_webhook_endpoint::WebhookEndpointId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: NotificationWebhookEndpointDeletedObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum NotificationWebhookEndpointDeletedObject {
    WebhookEndpoint,
}

impl NotificationWebhookEndpointDeletedObject {
    pub fn as_str(self) -> &'static str {
        use NotificationWebhookEndpointDeletedObject::*;
        match self {
            WebhookEndpoint => "webhook_endpoint",
        }
    }
}

impl std::str::FromStr for NotificationWebhookEndpointDeletedObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NotificationWebhookEndpointDeletedObject::*;
        match s {
            "webhook_endpoint" => Ok(WebhookEndpoint),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for NotificationWebhookEndpointDeletedObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for NotificationWebhookEndpointDeletedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for NotificationWebhookEndpointDeletedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for NotificationWebhookEndpointDeletedObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NotificationWebhookEndpointDeletedObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for NotificationWebhookEndpointDeletedObject"))
    }
}
impl stripe_types::Object for NotificationWebhookEndpointDeleted {
    type Id = stripe_misc::notification_webhook_endpoint::WebhookEndpointId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
