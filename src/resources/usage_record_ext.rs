use serde::{Deserialize, Serialize};

use crate::{Client, Response, SubscriptionItemId, Timestamp, UsageRecord};

impl UsageRecord {
    pub fn create(
        client: &Client,
        subscription_item_id: &SubscriptionItemId,
        params: CreateUsageRecord,
    ) -> Response<UsageRecord> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(
            &format!("/subscription_items/{}/usage_records", subscription_item_id),
            &params,
        )
    }
}

/// The parameters for `UsageRecord::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateUsageRecord {
    /// The usage quantity for the specified timestamp.
    pub quantity: u64,
    /// Valid values are `increment` (default) or `set`.
    /// When using `increment` the specified `quantity` will be added to the usage at the specified timestamp.
    /// The `set` action will overwrite the usage quantity at that timestamp.
    /// If the subscription has [billing thresholds](https://stripe.com/docs/api/subscriptions/object#subscription_object-billing_thresholds),
    /// `increment` is the only allowed value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<UsageRecordAction>,
    /// The timestamp for the usage event.
    /// This timestamp must be within the current billing period of the subscription of the provided `subscription_item`,
    /// and must not be in the future. When passing `"now"`, Stripe records usage for the current time.
    /// Default is `"now"` if a value is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,
}

/// An enum representing the possible values of a `UsageRecord`'s `account` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UsageRecordAction {
    Increment,
    Set,
}
