// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{PlanId, SubscriptionId, SubscriptionItemId};
use crate::params::{Deleted, Expand, List, Metadata, Object, Timestamp};
use crate::resources::{Plan, SubscriptionItemBillingThresholds, TaxRate};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionItem".
///
/// For more details see [https://stripe.com/docs/api/subscription_items/object](https://stripe.com/docs/api/subscription_items/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItem {
    /// Unique identifier for the object.
    pub id: SubscriptionItemId,

    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,

    /// The [quantity](https://stripe.com/docs/subscriptions/quantities) of the plan to which the customer should be subscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The `subscription` this `subscription_item` belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    /// The tax rates which apply to this `subscription_item`.
    ///
    /// When set, the `default_tax_rates` on the subscription do not apply to this `subscription_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
}

impl SubscriptionItem {
    /// Returns a list of your subscription items for a given subscription.
    pub fn list(
        client: &Client,
        params: ListSubscriptionItems<'_>,
    ) -> Response<List<SubscriptionItem>> {
        client.get_query("/subscription_items", &params)
    }

    /// Retrieves the invoice item with the given ID.
    pub fn retrieve(
        client: &Client,
        id: &SubscriptionItemId,
        expand: &[&str],
    ) -> Response<SubscriptionItem> {
        client.get_query(&format!("/subscription_items/{}", id), &Expand { expand })
    }

    /// Updates the plan or quantity of an item on a current subscription.
    pub fn update(
        client: &Client,
        id: &SubscriptionItemId,
        params: UpdateSubscriptionItem<'_>,
    ) -> Response<SubscriptionItem> {
        client.post_form(&format!("/subscription_items/{}", id), &params)
    }

    /// Deletes an item from the subscription.
    ///
    /// Removing a subscription item from a subscription will not cancel the subscription.
    pub fn delete(
        client: &Client,
        id: &SubscriptionItemId,
    ) -> Response<Deleted<SubscriptionItemId>> {
        client.delete(&format!("/subscription_items/{}", id))
    }
}

impl Object for SubscriptionItem {
    type Id = SubscriptionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_item"
    }
}

/// The parameters for `SubscriptionItem::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListSubscriptionItems<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a SubscriptionItemId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SubscriptionItemId>,

    /// The ID of the subscription whose items will be retrieved.
    pub subscription: SubscriptionId,
}

impl<'a> ListSubscriptionItems<'a> {
    pub fn new(subscription: SubscriptionId) -> Self {
        ListSubscriptionItems {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            subscription,
        }
    }
}

/// The parameters for `SubscriptionItem::update`.
#[derive(Clone, Debug, Serialize)]
pub struct UpdateSubscriptionItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpdateSubscriptionItemBillingThresholds>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The identifier of the new plan for this subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanId>,

    /// Flag indicating whether to [prorate](https://stripe.com/docs/billing/subscriptions/prorations) switching plans during a billing cycle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,

    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](#retrieve_customer_invoice) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<Timestamp>,

    /// The quantity you'd like to apply to the subscription item you're creating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

impl<'a> UpdateSubscriptionItem<'a> {
    pub fn new() -> Self {
        UpdateSubscriptionItem {
            billing_thresholds: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            plan: Default::default(),
            prorate: Default::default(),
            proration_date: Default::default(),
            quantity: Default::default(),
            tax_rates: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionItemBillingThresholds {
    pub usage_gte: i64,
}
