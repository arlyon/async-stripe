use crate::config::{Client, Response};
use crate::ids::SubscriptionId;
use crate::resources::{CreateSubscriptionItems, Subscription};
use serde_derive::Serialize;

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
    /// For more details see https://stripe.com/docs/api#cancel_subscription.
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
        Self {
            billing_thresholds: Default::default(),
            metadata: Default::default(),
            plan: Default::default(),
            price: Default::default(),
            price_data: Default::default(),
            quantity: Default::default(),
            tax_rates: Default::default(),
        }
    }
}
