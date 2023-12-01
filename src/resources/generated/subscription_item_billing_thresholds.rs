// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionItemBillingThresholds".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionItemBillingThresholds {
    /// Usage threshold that triggers the subscription to create an invoice.
    pub usage_gte: Option<i64>,
}
