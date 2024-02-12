use serde::Serialize;

use crate::client::{Client, Response};
use crate::ids::SubscriptionId;
use crate::params::SearchList;
use crate::resources::{CreateSubscriptionItems, Subscription};
use crate::CancellationDetails;

#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_details: Option<CancellationDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
}

impl CancelSubscription {
    pub fn new() -> CancelSubscription {
        CancelSubscription { cancellation_details: None, invoice_now: None, prorate: None }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SubscriptionSearchParams<'a> {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    pub expand: &'a [&'a str],
}

impl<'a> SubscriptionSearchParams<'a> {
    pub fn new() -> SubscriptionSearchParams<'a> {
        SubscriptionSearchParams { query: String::new(), limit: None, page: None, expand: &[] }
    }
}

impl Subscription {
    /// Cancels a subscription.
    ///
    /// For more details see <https://stripe.com/docs/api#cancel_subscription>.
    pub fn cancel(
        client: &Client,
        subscription_id: &SubscriptionId,
        params: CancelSubscription,
    ) -> Response<Subscription> {
        client.delete_query(&format!("/subscriptions/{}", subscription_id), params)
    }
    /// Searches for a subscription.
    ///
    /// For more details see <https://stripe.com/docs/api/subscriptions/search>.
    pub fn search(
        client: &Client,
        params: SubscriptionSearchParams,
    ) -> Response<SearchList<Subscription>> {
        client.get_query("/subscriptions/search", params)
    }
}

impl CreateSubscriptionItems {
    pub fn new() -> Self {
        Default::default()
    }
}
