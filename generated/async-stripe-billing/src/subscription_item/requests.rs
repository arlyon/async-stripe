use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct DeleteSubscriptionItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    clear_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_behavior: Option<stripe_shared::PaymentBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<stripe_shared::ProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_date: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeleteSubscriptionItemBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeleteSubscriptionItemBuilder").finish_non_exhaustive()
    }
}
impl DeleteSubscriptionItemBuilder {
    fn new() -> Self {
        Self {
            clear_usage: None,
            payment_behavior: None,
            proration_behavior: None,
            proration_date: None,
        }
    }
}
/// Deletes an item from the subscription.
/// Removing a subscription item from a subscription will not cancel the subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeleteSubscriptionItem {
    inner: DeleteSubscriptionItemBuilder,
    item: stripe_shared::SubscriptionItemId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeleteSubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeleteSubscriptionItem").finish_non_exhaustive()
    }
}
impl DeleteSubscriptionItem {
    /// Construct a new `DeleteSubscriptionItem`.
    pub fn new(item: impl Into<stripe_shared::SubscriptionItemId>) -> Self {
        Self { item: item.into(), inner: DeleteSubscriptionItemBuilder::new() }
    }
    /// Delete all usage for the given subscription item.
    /// Allowed only when the current plan's `usage_type` is `metered`.
    pub fn clear_usage(mut self, clear_usage: impl Into<bool>) -> Self {
        self.inner.clear_usage = Some(clear_usage.into());
        self
    }
    /// Controls how Stripe handles payment when a subscription update requires payment and `collection_method=charge_automatically`.
    pub fn payment_behavior(
        mut self,
        payment_behavior: impl Into<stripe_shared::PaymentBehavior>,
    ) -> Self {
        self.inner.payment_behavior = Some(payment_behavior.into());
        self
    }
    /// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    pub fn proration_behavior(
        mut self,
        proration_behavior: impl Into<stripe_shared::ProrationBehavior>,
    ) -> Self {
        self.inner.proration_behavior = Some(proration_behavior.into());
        self
    }
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](/api/invoices/create_preview) endpoint.
    pub fn proration_date(mut self, proration_date: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.proration_date = Some(proration_date.into());
        self
    }
}
impl DeleteSubscriptionItem {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeleteSubscriptionItem {
    type Output = stripe_shared::DeletedSubscriptionItem;

    fn build(&self) -> RequestBuilder {
        let item = &self.item;
        RequestBuilder::new(StripeMethod::Delete, format!("/subscription_items/{item}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListSubscriptionItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    subscription: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListSubscriptionItemBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListSubscriptionItemBuilder").finish_non_exhaustive()
    }
}
impl ListSubscriptionItemBuilder {
    fn new(subscription: impl Into<String>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            subscription: subscription.into(),
        }
    }
}
/// Returns a list of your subscription items for a given subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListSubscriptionItem {
    inner: ListSubscriptionItemBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListSubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListSubscriptionItem").finish_non_exhaustive()
    }
}
impl ListSubscriptionItem {
    /// Construct a new `ListSubscriptionItem`.
    pub fn new(subscription: impl Into<String>) -> Self {
        Self { inner: ListSubscriptionItemBuilder::new(subscription.into()) }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl ListSubscriptionItem {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::SubscriptionItem>>
    {
        stripe_client_core::ListPaginator::new_list("/subscription_items", &self.inner)
    }
}

impl StripeRequest for ListSubscriptionItem {
    type Output = stripe_types::List<stripe_shared::SubscriptionItem>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/subscription_items").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveSubscriptionItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveSubscriptionItemBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveSubscriptionItemBuilder").finish_non_exhaustive()
    }
}
impl RetrieveSubscriptionItemBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the subscription item with the given ID.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveSubscriptionItem {
    inner: RetrieveSubscriptionItemBuilder,
    item: stripe_shared::SubscriptionItemId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveSubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveSubscriptionItem").finish_non_exhaustive()
    }
}
impl RetrieveSubscriptionItem {
    /// Construct a new `RetrieveSubscriptionItem`.
    pub fn new(item: impl Into<stripe_shared::SubscriptionItemId>) -> Self {
        Self { item: item.into(), inner: RetrieveSubscriptionItemBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveSubscriptionItem {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveSubscriptionItem {
    type Output = stripe_shared::SubscriptionItem;

    fn build(&self) -> RequestBuilder {
        let item = &self.item;
        RequestBuilder::new(StripeMethod::Get, format!("/subscription_items/{item}"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateSubscriptionItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_thresholds: Option<ItemBillingThresholdsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_behavior: Option<stripe_shared::PaymentBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data: Option<RecurringPriceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<stripe_shared::ProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    subscription: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionItemBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionItemBuilder").finish_non_exhaustive()
    }
}
impl CreateSubscriptionItemBuilder {
    fn new(subscription: impl Into<String>) -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            expand: None,
            metadata: None,
            payment_behavior: None,
            plan: None,
            price: None,
            price_data: None,
            proration_behavior: None,
            proration_date: None,
            quantity: None,
            subscription: subscription.into(),
            tax_rates: None,
        }
    }
}
/// Adds a new item to an existing subscription. No existing items will be changed or replaced.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionItem {
    inner: CreateSubscriptionItemBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionItem").finish_non_exhaustive()
    }
}
impl CreateSubscriptionItem {
    /// Construct a new `CreateSubscriptionItem`.
    pub fn new(subscription: impl Into<String>) -> Self {
        Self { inner: CreateSubscriptionItemBuilder::new(subscription.into()) }
    }
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    pub fn billing_thresholds(
        mut self,
        billing_thresholds: impl Into<ItemBillingThresholdsParam>,
    ) -> Self {
        self.inner.billing_thresholds = Some(billing_thresholds.into());
        self
    }
    /// The coupons to redeem into discounts for the subscription item.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Controls how Stripe handles payment when a subscription update requires payment and `collection_method=charge_automatically`.
    pub fn payment_behavior(
        mut self,
        payment_behavior: impl Into<stripe_shared::PaymentBehavior>,
    ) -> Self {
        self.inner.payment_behavior = Some(payment_behavior.into());
        self
    }
    /// The identifier of the plan to add to the subscription.
    pub fn plan(mut self, plan: impl Into<String>) -> Self {
        self.inner.plan = Some(plan.into());
        self
    }
    /// The ID of the price object.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.inner.price = Some(price.into());
        self
    }
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
    pub fn price_data(mut self, price_data: impl Into<RecurringPriceData>) -> Self {
        self.inner.price_data = Some(price_data.into());
        self
    }
    /// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    pub fn proration_behavior(
        mut self,
        proration_behavior: impl Into<stripe_shared::ProrationBehavior>,
    ) -> Self {
        self.inner.proration_behavior = Some(proration_behavior.into());
        self
    }
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](/api/invoices/create_preview) endpoint.
    pub fn proration_date(mut self, proration_date: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.proration_date = Some(proration_date.into());
        self
    }
    /// The quantity you'd like to apply to the subscription item you're creating.
    pub fn quantity(mut self, quantity: impl Into<u64>) -> Self {
        self.inner.quantity = Some(quantity.into());
        self
    }
    /// A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://docs.stripe.com/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    pub fn tax_rates(mut self, tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.tax_rates = Some(tax_rates.into());
        self
    }
}
impl CreateSubscriptionItem {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateSubscriptionItem {
    type Output = stripe_shared::SubscriptionItem;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/subscription_items").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateSubscriptionItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_thresholds: Option<ItemBillingThresholdsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    off_session: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_behavior: Option<stripe_shared::PaymentBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data: Option<RecurringPriceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<stripe_shared::ProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionItemBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionItemBuilder").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionItemBuilder {
    fn new() -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            expand: None,
            metadata: None,
            off_session: None,
            payment_behavior: None,
            plan: None,
            price: None,
            price_data: None,
            proration_behavior: None,
            proration_date: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
/// Updates the plan or quantity of an item on a current subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionItem {
    inner: UpdateSubscriptionItemBuilder,
    item: stripe_shared::SubscriptionItemId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionItem").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionItem {
    /// Construct a new `UpdateSubscriptionItem`.
    pub fn new(item: impl Into<stripe_shared::SubscriptionItemId>) -> Self {
        Self { item: item.into(), inner: UpdateSubscriptionItemBuilder::new() }
    }
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    pub fn billing_thresholds(
        mut self,
        billing_thresholds: impl Into<ItemBillingThresholdsParam>,
    ) -> Self {
        self.inner.billing_thresholds = Some(billing_thresholds.into());
        self
    }
    /// The coupons to redeem into discounts for the subscription item.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    /// Defaults to `false` (on-session).
    pub fn off_session(mut self, off_session: impl Into<bool>) -> Self {
        self.inner.off_session = Some(off_session.into());
        self
    }
    /// Controls how Stripe handles payment when a subscription update requires payment and `collection_method=charge_automatically`.
    pub fn payment_behavior(
        mut self,
        payment_behavior: impl Into<stripe_shared::PaymentBehavior>,
    ) -> Self {
        self.inner.payment_behavior = Some(payment_behavior.into());
        self
    }
    /// The identifier of the new plan for this subscription item.
    pub fn plan(mut self, plan: impl Into<String>) -> Self {
        self.inner.plan = Some(plan.into());
        self
    }
    /// The ID of the price object.
    /// One of `price` or `price_data` is required.
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.inner.price = Some(price.into());
        self
    }
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    pub fn price_data(mut self, price_data: impl Into<RecurringPriceData>) -> Self {
        self.inner.price_data = Some(price_data.into());
        self
    }
    /// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    pub fn proration_behavior(
        mut self,
        proration_behavior: impl Into<stripe_shared::ProrationBehavior>,
    ) -> Self {
        self.inner.proration_behavior = Some(proration_behavior.into());
        self
    }
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](/api/invoices/create_preview) endpoint.
    pub fn proration_date(mut self, proration_date: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.proration_date = Some(proration_date.into());
        self
    }
    /// The quantity you'd like to apply to the subscription item you're creating.
    pub fn quantity(mut self, quantity: impl Into<u64>) -> Self {
        self.inner.quantity = Some(quantity.into());
        self
    }
    /// A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://docs.stripe.com/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    pub fn tax_rates(mut self, tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.tax_rates = Some(tax_rates.into());
        self
    }
}
impl UpdateSubscriptionItem {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateSubscriptionItem {
    type Output = stripe_shared::SubscriptionItem;

    fn build(&self) -> RequestBuilder {
        let item = &self.item;
        RequestBuilder::new(StripeMethod::Post, format!("/subscription_items/{item}"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ItemBillingThresholdsParam {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://docs.stripe.com/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ItemBillingThresholdsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ItemBillingThresholdsParam").finish_non_exhaustive()
    }
}
impl ItemBillingThresholdsParam {
    pub fn new(usage_gte: impl Into<i64>) -> Self {
        Self { usage_gte: usage_gte.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DiscountsDataParam {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DiscountsDataParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DiscountsDataParam").finish_non_exhaustive()
    }
}
impl DiscountsDataParam {
    pub fn new() -> Self {
        Self { coupon: None, discount: None, promotion_code: None }
    }
}
impl Default for DiscountsDataParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RecurringAdhoc {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RecurringAdhoc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RecurringAdhoc").finish_non_exhaustive()
    }
}
impl RecurringAdhoc {
    pub fn new(interval: impl Into<stripe_shared::PlanInterval>) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RecurringPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: RecurringAdhoc,
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RecurringPriceData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RecurringPriceData").finish_non_exhaustive()
    }
}
impl RecurringPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<RecurringAdhoc>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Controls how Stripe handles payment when a subscription update requires payment and `collection_method=charge_automatically`.
pub type DeleteSubscriptionItemPaymentBehavior = stripe_shared::PaymentBehavior;
/// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
pub type DeleteSubscriptionItemProrationBehavior = stripe_shared::ProrationBehavior;
/// Controls how Stripe handles payment when a subscription update requires payment and `collection_method=charge_automatically`.
pub type CreateSubscriptionItemPaymentBehavior = stripe_shared::PaymentBehavior;
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
pub type CreateSubscriptionItemPriceDataRecurringInterval = stripe_shared::PlanInterval;
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type CreateSubscriptionItemPriceDataTaxBehavior = stripe_shared::PriceTaxBehavior;
/// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
pub type CreateSubscriptionItemProrationBehavior = stripe_shared::ProrationBehavior;
/// Controls how Stripe handles payment when a subscription update requires payment and `collection_method=charge_automatically`.
pub type UpdateSubscriptionItemPaymentBehavior = stripe_shared::PaymentBehavior;
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
pub type UpdateSubscriptionItemPriceDataRecurringInterval = stripe_shared::PlanInterval;
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type UpdateSubscriptionItemPriceDataTaxBehavior = stripe_shared::PriceTaxBehavior;
/// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
pub type UpdateSubscriptionItemProrationBehavior = stripe_shared::ProrationBehavior;
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// Pass an empty string to remove previously-defined thresholds.
pub type CreateSubscriptionItemBillingThresholds = ItemBillingThresholdsParam;
/// The coupons to redeem into discounts for the subscription item.
pub type CreateSubscriptionItemDiscounts = DiscountsDataParam;
/// The recurring components of a price such as `interval` and `interval_count`.
pub type CreateSubscriptionItemPriceDataRecurring = RecurringAdhoc;
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// Pass an empty string to remove previously-defined thresholds.
pub type UpdateSubscriptionItemBillingThresholds = ItemBillingThresholdsParam;
/// The coupons to redeem into discounts for the subscription item.
pub type UpdateSubscriptionItemDiscounts = DiscountsDataParam;
/// The recurring components of a price such as `interval` and `interval_count`.
pub type UpdateSubscriptionItemPriceDataRecurring = RecurringAdhoc;
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
pub type CreateSubscriptionItemPriceData = RecurringPriceData;
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
/// One of `price` or `price_data` is required.
pub type UpdateSubscriptionItemPriceData = RecurringPriceData;
