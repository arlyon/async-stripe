use serde::{Deserialize, Serialize};

use crate::params::{List, Object, Paginable};
use crate::{
    Client, Response, SubscriptionItemId, Timestamp, UsageRecord, UsageRecordSummary,
    UsageRecordSummaryId,
};

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

impl UsageRecordSummary {
    /// For the specified subscription item, returns a list of summary objects. Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).
    ///
    /// The list is sorted in reverse-chronological order (newest first). The first list item represents the most current usage period that hasn’t ended yet. Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.
    pub fn list(
        client: &Client,
        subscription_item_id: &SubscriptionItemId,
        params: &ListUsageRecordSummaries,
    ) -> Response<List<UsageRecordSummary>> {
        client.get_query(
            &format!("/subscription_items/{}/usage_record_summaries", subscription_item_id),
            &params,
        )
    }
}

/// For the specified subscription item, returns a list of summary objects. Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).
///
/// The list is sorted in reverse-chronological order (newest first). The first list item represents the most current usage period that hasn’t ended yet. Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.
/// For more details see [https://stripe.com/docs/api/usage_records/subscription_item_summary_list](https://stripe.com/docs/api/usage_records/subscription_item_summary_list).
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListUsageRecordSummaries {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<UsageRecordSummaryId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<UsageRecordSummaryId>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

impl ListUsageRecordSummaries {
    pub fn new() -> Self {
        ListUsageRecordSummaries::default()
    }
}

impl Paginable for ListUsageRecordSummaries {
    type O = UsageRecordSummary;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
