// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionsResourceBillingMode".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionsResourceBillingMode {

    /// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
    #[serde(rename = "type")]
    pub type_: SubscriptionsResourceBillingModeType,

    /// Details on when the current billing_mode was adopted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// An enum representing the possible values of an `SubscriptionsResourceBillingMode`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsResourceBillingModeType {
    Classic,
    Flexible,
}

impl SubscriptionsResourceBillingModeType {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionsResourceBillingModeType::Classic => "classic",
            SubscriptionsResourceBillingModeType::Flexible => "flexible",
        }
    }
}

impl AsRef<str> for SubscriptionsResourceBillingModeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionsResourceBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionsResourceBillingModeType {
    fn default() -> Self {
        Self::Classic
    }
}
