// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionItemBillingThresholds".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionItemBillingThresholds {
    /// Usage threshold that triggers the subscription to create an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_gte: Option<Box<i64>>,
}
