use serde::Serialize;

use crate::CancellationDetails;
use crate::client::{Client, Response};
use crate::ids::SubscriptionId;
use crate::resources::{CreateSubscriptionItems, Subscription};

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
}

impl CreateSubscriptionItems {
    pub fn new() -> Self {
        Default::default()
    }
}
