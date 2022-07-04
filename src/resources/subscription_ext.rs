use serde::Serialize;

use crate::client::{Client, Response};
use crate::ids::SubscriptionId;
use crate::resources::{CreateSubscriptionItems, Subscription};

#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_period_end: Option<bool>,
}

impl CancelSubscription {
    pub fn new() -> CancelSubscription {
        CancelSubscription { at_period_end: None }
    }
}

impl Subscription {
    /// Cancels a subscription.
    ///
    /// For more details see <https://stripe.com/docs/api#cancel_subscription>.
    pub fn cancel<'a>(
        client: &'a Client,
        subscription_id: &'_ SubscriptionId,
        params: CancelSubscription,
    ) -> Response<'a, Subscription> {
        client.delete_query(&format!("/subscriptions/{}", subscription_id), params)
    }
}

impl CreateSubscriptionItems {
    pub fn new() -> Self {
        Default::default()
    }
}
