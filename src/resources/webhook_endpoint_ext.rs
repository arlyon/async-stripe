use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `WebhookEndpoint`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEndpointStatus {
    Disabled,
    Enabled,
}

impl WebhookEndpointStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            WebhookEndpointStatus::Disabled => "disabled",
            WebhookEndpointStatus::Enabled => "enabled",
        }
    }
}

impl AsRef<str> for WebhookEndpointStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WebhookEndpointStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
