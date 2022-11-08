// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionBillingThresholds".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionBillingThresholds {
    /// Monetary threshold that triggers the subscription to create an invoice.
    pub amount_gte: Option<i64>,

    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    ///
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    /// This value may not be `true` if the subscription contains items with plans that have `aggregate_usage=last_ever`.
    pub reset_billing_cycle_anchor: Option<bool>,
}
