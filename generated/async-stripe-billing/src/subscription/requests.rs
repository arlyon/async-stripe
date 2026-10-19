use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CancelSubscriptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    cancellation_details: Option<CancellationDetailsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prorate: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancelSubscriptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancelSubscriptionBuilder").finish_non_exhaustive()
    }
}
impl CancelSubscriptionBuilder {
    fn new() -> Self {
        Self { cancellation_details: None, expand: None, invoice_now: None, prorate: None }
    }
}
/// Cancels a customer’s subscription immediately.
/// The customer won’t be charged again for the subscription.
/// After it’s canceled, the subscription is largely immutable.
/// You can still update its <a href="/metadata">metadata</a> and `cancellation_details`.
///
/// Any pending invoice items that you’ve created are still charged at the end of the period, unless manually <a href="/api/invoiceitems/delete">deleted</a>.
/// If you’ve set the subscription to cancel at the end of the period, any pending prorations are also left in place and collected at the end of the period.
/// But if the subscription is set to cancel immediately, pending prorations are removed if `invoice_now` and `prorate` are both set to false.
///
/// By default, upon subscription cancellation, Stripe stops automatic collection of all finalized invoices for the customer.
/// This is intended to prevent unexpected payment attempts after the customer has canceled a subscription.
/// However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed.
/// Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CancelSubscription {
    inner: CancelSubscriptionBuilder,
    subscription_exposed_id: stripe_shared::SubscriptionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancelSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancelSubscription").finish_non_exhaustive()
    }
}
impl CancelSubscription {
    /// Construct a new `CancelSubscription`.
    pub fn new(subscription_exposed_id: impl Into<stripe_shared::SubscriptionId>) -> Self {
        Self {
            subscription_exposed_id: subscription_exposed_id.into(),
            inner: CancelSubscriptionBuilder::new(),
        }
    }
    /// Details about why this subscription was cancelled
    pub fn cancellation_details(
        mut self,
        cancellation_details: impl Into<CancellationDetailsParam>,
    ) -> Self {
        self.inner.cancellation_details = Some(cancellation_details.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Will generate a final invoice that invoices for any un-invoiced metered usage and new/pending proration invoice items.
    /// Defaults to `false`.
    pub fn invoice_now(mut self, invoice_now: impl Into<bool>) -> Self {
        self.inner.invoice_now = Some(invoice_now.into());
        self
    }
    /// Will generate a proration invoice item that credits remaining unused time until the subscription period end.
    /// Defaults to `false`.
    pub fn prorate(mut self, prorate: impl Into<bool>) -> Self {
        self.inner.prorate = Some(prorate.into());
        self
    }
}
impl CancelSubscription {
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

impl StripeRequest for CancelSubscription {
    type Output = stripe_shared::Subscription;

    fn build(&self) -> RequestBuilder {
        let subscription_exposed_id = &self.subscription_exposed_id;
        RequestBuilder::new(
            StripeMethod::Delete,
            format!("/subscriptions/{subscription_exposed_id}"),
        )
        .form(&self.inner)
    }
}
/// Removes the currently applied discount on a subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeleteDiscountSubscription {
    subscription_exposed_id: stripe_shared::SubscriptionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeleteDiscountSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeleteDiscountSubscription").finish_non_exhaustive()
    }
}
impl DeleteDiscountSubscription {
    /// Construct a new `DeleteDiscountSubscription`.
    pub fn new(subscription_exposed_id: impl Into<stripe_shared::SubscriptionId>) -> Self {
        Self { subscription_exposed_id: subscription_exposed_id.into() }
    }
}
impl DeleteDiscountSubscription {
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

impl StripeRequest for DeleteDiscountSubscription {
    type Output = stripe_shared::DeletedDiscount;

    fn build(&self) -> RequestBuilder {
        let subscription_exposed_id = &self.subscription_exposed_id;
        RequestBuilder::new(
            StripeMethod::Delete,
            format!("/subscriptions/{subscription_exposed_id}/discount"),
        )
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListSubscriptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<ListSubscriptionAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_method: Option<stripe_shared::CollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_period_end: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_period_start: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ListSubscriptionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_clock: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListSubscriptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListSubscriptionBuilder").finish_non_exhaustive()
    }
}
impl ListSubscriptionBuilder {
    fn new() -> Self {
        Self {
            automatic_tax: None,
            collection_method: None,
            created: None,
            current_period_end: None,
            current_period_start: None,
            customer: None,
            customer_account: None,
            ending_before: None,
            expand: None,
            limit: None,
            plan: None,
            price: None,
            starting_after: None,
            status: None,
            test_clock: None,
        }
    }
}
/// Filter subscriptions by their automatic tax settings.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListSubscriptionAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListSubscriptionAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListSubscriptionAutomaticTax").finish_non_exhaustive()
    }
}
impl ListSubscriptionAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// The status of the subscriptions to retrieve.
/// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
/// Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://docs.stripe.com/billing/subscriptions/overview#subscription-statuses).
/// Passing in a value of `all` will return subscriptions of all statuses.
/// If no value is supplied, all subscriptions that have not been canceled are returned.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListSubscriptionStatus {
    Active,
    All,
    Canceled,
    Ended,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Paused,
    Trialing,
    Unpaid,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListSubscriptionStatus {
    pub fn as_str(&self) -> &str {
        use ListSubscriptionStatus::*;
        match self {
            Active => "active",
            All => "all",
            Canceled => "canceled",
            Ended => "ended",
            Incomplete => "incomplete",
            IncompleteExpired => "incomplete_expired",
            PastDue => "past_due",
            Paused => "paused",
            Trialing => "trialing",
            Unpaid => "unpaid",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListSubscriptionStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListSubscriptionStatus::*;
        match s {
            "active" => Ok(Active),
            "all" => Ok(All),
            "canceled" => Ok(Canceled),
            "ended" => Ok(Ended),
            "incomplete" => Ok(Incomplete),
            "incomplete_expired" => Ok(IncompleteExpired),
            "past_due" => Ok(PastDue),
            "paused" => Ok(Paused),
            "trialing" => Ok(Trialing),
            "unpaid" => Ok(Unpaid),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ListSubscriptionStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ListSubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ListSubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListSubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ListSubscriptionStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ListSubscriptionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListSubscriptionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// By default, returns a list of subscriptions that have not been canceled.
/// In order to list canceled subscriptions, specify `status=canceled`.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListSubscription {
    inner: ListSubscriptionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListSubscription").finish_non_exhaustive()
    }
}
impl ListSubscription {
    /// Construct a new `ListSubscription`.
    pub fn new() -> Self {
        Self { inner: ListSubscriptionBuilder::new() }
    }
    /// Filter subscriptions by their automatic tax settings.
    pub fn automatic_tax(mut self, automatic_tax: impl Into<ListSubscriptionAutomaticTax>) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// The collection method of the subscriptions to retrieve.
    /// Either `charge_automatically` or `send_invoice`.
    pub fn collection_method(
        mut self,
        collection_method: impl Into<stripe_shared::CollectionMethod>,
    ) -> Self {
        self.inner.collection_method = Some(collection_method.into());
        self
    }
    /// Only return subscriptions that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return subscriptions whose minimum item current_period_end falls within the given date interval.
    pub fn current_period_end(
        mut self,
        current_period_end: impl Into<stripe_types::RangeQueryTs>,
    ) -> Self {
        self.inner.current_period_end = Some(current_period_end.into());
        self
    }
    /// Only return subscriptions whose maximum item current_period_start falls within the given date interval.
    pub fn current_period_start(
        mut self,
        current_period_start: impl Into<stripe_types::RangeQueryTs>,
    ) -> Self {
        self.inner.current_period_start = Some(current_period_start.into());
        self
    }
    /// The ID of the customer whose subscriptions you're retrieving.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// The ID of the account representing the customer whose subscriptions you're retrieving.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
        self
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
    /// The ID of the plan whose subscriptions will be retrieved.
    pub fn plan(mut self, plan: impl Into<String>) -> Self {
        self.inner.plan = Some(plan.into());
        self
    }
    /// Filter for subscriptions that contain this recurring price ID.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.inner.price = Some(price.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// The status of the subscriptions to retrieve.
    /// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
    /// Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://docs.stripe.com/billing/subscriptions/overview#subscription-statuses).
    /// Passing in a value of `all` will return subscriptions of all statuses.
    /// If no value is supplied, all subscriptions that have not been canceled are returned.
    pub fn status(mut self, status: impl Into<ListSubscriptionStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
    /// Filter for subscriptions that are associated with the specified test clock.
    /// The response will not include subscriptions with test clocks if this and the customer parameter is not set.
    pub fn test_clock(mut self, test_clock: impl Into<String>) -> Self {
        self.inner.test_clock = Some(test_clock.into());
        self
    }
}
impl Default for ListSubscription {
    fn default() -> Self {
        Self::new()
    }
}
impl ListSubscription {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Subscription>> {
        stripe_client_core::ListPaginator::new_list("/subscriptions", &self.inner)
    }
}

impl StripeRequest for ListSubscription {
    type Output = stripe_types::List<stripe_shared::Subscription>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/subscriptions").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveSubscriptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveSubscriptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveSubscriptionBuilder").finish_non_exhaustive()
    }
}
impl RetrieveSubscriptionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the subscription with the given ID.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveSubscription {
    inner: RetrieveSubscriptionBuilder,
    subscription_exposed_id: stripe_shared::SubscriptionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveSubscription").finish_non_exhaustive()
    }
}
impl RetrieveSubscription {
    /// Construct a new `RetrieveSubscription`.
    pub fn new(subscription_exposed_id: impl Into<stripe_shared::SubscriptionId>) -> Self {
        Self {
            subscription_exposed_id: subscription_exposed_id.into(),
            inner: RetrieveSubscriptionBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveSubscription {
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

impl StripeRequest for RetrieveSubscription {
    type Output = stripe_shared::Subscription;

    fn build(&self) -> RequestBuilder {
        let subscription_exposed_id = &self.subscription_exposed_id;
        RequestBuilder::new(StripeMethod::Get, format!("/subscriptions/{subscription_exposed_id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct SearchSubscriptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<String>,
    query: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SearchSubscriptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SearchSubscriptionBuilder").finish_non_exhaustive()
    }
}
impl SearchSubscriptionBuilder {
    fn new(query: impl Into<String>) -> Self {
        Self { expand: None, limit: None, page: None, query: query.into() }
    }
}
/// Search for subscriptions you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
/// Under normal operating.
/// conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up.
/// to an hour behind during outages. Search functionality is not available to merchants in India.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SearchSubscription {
    inner: SearchSubscriptionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SearchSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SearchSubscription").finish_non_exhaustive()
    }
}
impl SearchSubscription {
    /// Construct a new `SearchSubscription`.
    pub fn new(query: impl Into<String>) -> Self {
        Self { inner: SearchSubscriptionBuilder::new(query.into()) }
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
    /// A cursor for pagination across multiple pages of results.
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.inner.page = Some(page.into());
        self
    }
}
impl SearchSubscription {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::SearchList<stripe_shared::Subscription>>
    {
        stripe_client_core::ListPaginator::new_search_list("/subscriptions/search", &self.inner)
    }
}

impl StripeRequest for SearchSubscription {
    type Output = stripe_types::SearchList<stripe_shared::Subscription>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/subscriptions/search").query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateSubscriptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    add_invoice_items: Option<Vec<AddInvoiceItemEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<CreateSubscriptionAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backdate_start_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_cycle_anchor: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_cycle_anchor_config: Option<CreateSubscriptionBillingCycleAnchorConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_mode: Option<CreateSubscriptionBillingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_schedules: Option<Vec<CreateSubscriptionBillingSchedules>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_thresholds: Option<BillingThresholdsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_at: Option<CreateSubscriptionCancelAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_at_period_end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_method: Option<stripe_shared::CollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_until_due: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_settings: Option<CreateSubscriptionInvoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CreateSubscriptionItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    off_session: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_behavior: Option<stripe_shared::PaymentBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_settings: Option<CreateSubscriptionPaymentSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_invoice_item_interval: Option<PendingInvoiceItemIntervalParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<stripe_shared::ProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<TransferDataSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_end: Option<CreateSubscriptionTrialEnd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_from_plan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_period_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_settings: Option<TrialSettingsConfig>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionBuilder").finish_non_exhaustive()
    }
}
impl CreateSubscriptionBuilder {
    fn new() -> Self {
        Self {
            add_invoice_items: None,
            application_fee_percent: None,
            automatic_tax: None,
            backdate_start_date: None,
            billing_cycle_anchor: None,
            billing_cycle_anchor_config: None,
            billing_mode: None,
            billing_schedules: None,
            billing_thresholds: None,
            cancel_at: None,
            cancel_at_period_end: None,
            collection_method: None,
            currency: None,
            customer: None,
            customer_account: None,
            days_until_due: None,
            default_payment_method: None,
            default_source: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            expand: None,
            invoice_settings: None,
            items: None,
            metadata: None,
            off_session: None,
            on_behalf_of: None,
            payment_behavior: None,
            payment_settings: None,
            pending_invoice_item_interval: None,
            proration_behavior: None,
            transfer_data: None,
            trial_end: None,
            trial_from_plan: None,
            trial_period_days: None,
            trial_settings: None,
        }
    }
}
/// Automatic tax settings for this subscription.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateSubscriptionAutomaticTaxLiability>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionAutomaticTax").finish_non_exhaustive()
    }
}
impl CreateSubscriptionAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionAutomaticTaxLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionAutomaticTaxLiability").finish_non_exhaustive()
    }
}
impl CreateSubscriptionAutomaticTaxLiability {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Mutually exclusive with billing_cycle_anchor and only valid with monthly and yearly price intervals.
/// When provided, the billing_cycle_anchor is set to the next occurrence of the day_of_month at the hour, minute, and second UTC.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionBillingCycleAnchorConfig {
    /// The day of the month the anchor should be. Ranges from 1 to 31.
    pub day_of_month: i64,
    /// The hour of the day the anchor should be. Ranges from 0 to 23.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<i64>,
    /// The minute of the hour the anchor should be. Ranges from 0 to 59.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<i64>,
    /// The month to start full cycle periods. Ranges from 1 to 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    /// The second of the minute the anchor should be. Ranges from 0 to 59.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionBillingCycleAnchorConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionBillingCycleAnchorConfig").finish_non_exhaustive()
    }
}
impl CreateSubscriptionBillingCycleAnchorConfig {
    pub fn new(day_of_month: impl Into<i64>) -> Self {
        Self {
            day_of_month: day_of_month.into(),
            hour: None,
            minute: None,
            month: None,
            second: None,
        }
    }
}
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionBillingMode {
    /// Configure behavior for flexible billing mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible: Option<FlexibleParams>,
    /// Controls the calculation and orchestration of prorations and invoices for subscriptions.
    /// If no value is passed, the default is `flexible`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateSubscriptionBillingModeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionBillingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionBillingMode").finish_non_exhaustive()
    }
}
impl CreateSubscriptionBillingMode {
    pub fn new(type_: impl Into<stripe_shared::CreateSubscriptionBillingModeType>) -> Self {
        Self { flexible: None, type_: type_.into() }
    }
}
/// An array of billing schedules, which allow you to bill customers in advance for multiple service periods.
/// Requires flexible billing mode and API version 2026-05-27.dahlia or later.
/// Learn more about [prebilling](https://docs.stripe.com/billing/subscriptions/prebilling).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionBillingSchedules {
    /// Configure billing schedule differently for individual subscription items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<Vec<BillingSchedulesAppliesTo>>,
    /// The end date for the billing schedule.
    /// You must not set this earlier than current period end for every applicable subscription item.
    pub bill_until: BillingSchedulesBillUntil,
    /// Specify a key for the billing schedule.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    /// If not provided, a unique key will be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionBillingSchedules {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionBillingSchedules").finish_non_exhaustive()
    }
}
impl CreateSubscriptionBillingSchedules {
    pub fn new(bill_until: impl Into<BillingSchedulesBillUntil>) -> Self {
        Self { applies_to: None, bill_until: bill_until.into(), key: None }
    }
}
/// A timestamp at which the subscription should cancel.
/// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
/// If set during a future period, this will always cause a proration for that period.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionCancelAt {
    MaxBilledUntil,
    MaxPeriodEnd,
    MinPeriodEnd,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionCancelAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionCancelAt").finish_non_exhaustive()
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionInvoiceSettings {
    /// The account tax IDs associated with the subscription.
    /// Will be set on invoices generated by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// A list of up to 4 custom fields to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldParams>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Footer to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateSubscriptionInvoiceSettingsIssuer>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionInvoiceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionInvoiceSettings").finish_non_exhaustive()
    }
}
impl CreateSubscriptionInvoiceSettings {
    pub fn new() -> Self {
        Self {
            account_tax_ids: None,
            custom_fields: None,
            description: None,
            footer: None,
            issuer: None,
        }
    }
}
impl Default for CreateSubscriptionInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionInvoiceSettingsIssuer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionInvoiceSettingsIssuer").finish_non_exhaustive()
    }
}
impl CreateSubscriptionInvoiceSettingsIssuer {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<RecurringPriceData>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://docs.stripe.com/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionItems").finish_non_exhaustive()
    }
}
impl CreateSubscriptionItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for CreateSubscriptionItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment settings to pass to invoices created by the subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettings {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptions>,
    /// The list of payment method types (e.g.
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    /// Should not be specified with payment_method_configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<stripe_shared::PaymentMethodTypes>>,
    /// Configure whether Stripe updates `subscription.default_payment_method` when payment succeeds.
    /// Defaults to `off` if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_default_payment_method: Option<stripe_shared::SaveDefaultPaymentMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettings").finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettings {
    pub fn new() -> Self {
        Self {
            payment_method_options: None,
            payment_method_types: None,
            save_default_payment_method: None,
        }
    }
}
impl Default for CreateSubscriptionPaymentSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment-method-specific configuration to provide to invoices created by the subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<InvoicePaymentMethodOptionsParam>,
    /// This sub-hash contains details about the Billie payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub billie: Option<miniserde::json::Value>,
    /// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance>,
    /// This sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// This sub-hash contains details about the PayTo payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsPayto>,
    /// This sub-hash contains details about the Pix payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<SubscriptionPaymentMethodOptionsParam>,
    /// This sub-hash contains details about the SEPA Direct Debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub sepa_debit: Option<miniserde::json::Value>,
    /// This sub-hash contains details about the UPI payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsUpi>,
    /// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettingsPaymentMethodOptions")
            .finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptions {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            bancontact: None,
            billie: None,
            card: None,
            customer_balance: None,
            konbini: None,
            payto: None,
            pix: None,
            sepa_debit: None,
            upi: None,
            us_bank_account: None,
        }
    }
}
impl Default for CreateSubscriptionPaymentSettingsPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<MandateOptionsParam>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit")
            .finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self { mandate_options: None, verification_method: None }
    }
}
impl Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions>,
    /// Selected network to process this Subscription on.
    /// Depends on the available networks of the card attached to the Subscription.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard")
            .finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self { mandate_options: None, network: None, request_three_d_secure: None }
    }
}
impl Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments, specified in the presentment currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_shared::AmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None }
    }
}
impl Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<BankTransferParam>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance")
            .finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    pub fn new() -> Self {
        Self { bank_transfer: None, funding_type: None }
    }
}
impl Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the PayTo payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsPayto {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettingsPaymentMethodOptionsPayto")
            .finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsPayto {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    /// The maximum amount that can be collected in a single invoice.
    /// If you don't specify a maximum, then there is no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The purpose for which payments are made. Has a default value based on your merchant category code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<stripe_shared::Purpose>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, purpose: None }
    }
}
impl Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the UPI payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsUpi {
    /// Configuration options for setting up an eMandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<InvoicePaymentMethodOptionsMandateOptionsParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettingsPaymentMethodOptionsUpi")
            .finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsUpi {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUpi {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<InvoiceLinkedAccountOptionsParam>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount")
            .finish_non_exhaustive()
    }
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self { financial_connections: None, verification_method: None }
    }
}
impl Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
/// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
/// The special value `now` can be provided to end the customer's trial immediately.
/// Can be at most two years from `billing_cycle_anchor`.
/// See [Using trial periods on subscriptions](https://docs.stripe.com/billing/subscriptions/trials) to learn more.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionTrialEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscriptionTrialEnd").finish_non_exhaustive()
    }
}
/// Creates a new subscription on an existing customer.
/// Each customer can have up to 500 active or scheduled subscriptions.
///
/// When you create a subscription with `collection_method=charge_automatically`, the first invoice is finalized as part of the request.
/// The `payment_behavior` parameter determines the exact behavior of the initial payment.
///
/// To start subscriptions where the first invoice always begins in a `draft` status, use [subscription schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules#managing) instead.
/// Schedules provide the flexibility to model more complex billing configurations that change over time.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSubscription {
    inner: CreateSubscriptionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSubscription").finish_non_exhaustive()
    }
}
impl CreateSubscription {
    /// Construct a new `CreateSubscription`.
    pub fn new() -> Self {
        Self { inner: CreateSubscriptionBuilder::new() }
    }
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
    /// You may pass up to 20 items.
    pub fn add_invoice_items(
        mut self,
        add_invoice_items: impl Into<Vec<AddInvoiceItemEntry>>,
    ) -> Self {
        self.inner.add_invoice_items = Some(add_invoice_items.into());
        self
    }
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    pub fn application_fee_percent(mut self, application_fee_percent: impl Into<f64>) -> Self {
        self.inner.application_fee_percent = Some(application_fee_percent.into());
        self
    }
    /// Automatic tax settings for this subscription.
    pub fn automatic_tax(
        mut self,
        automatic_tax: impl Into<CreateSubscriptionAutomaticTax>,
    ) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// A past timestamp to backdate the subscription's start date to.
    /// If set, the first invoice will contain line items for the timespan between the start date and the current time.
    /// Can be combined with trials and the billing cycle anchor.
    pub fn backdate_start_date(
        mut self,
        backdate_start_date: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        self.inner.backdate_start_date = Some(backdate_start_date.into());
        self
    }
    /// A future timestamp in UTC format to anchor the subscription's [billing cycle](https://docs.stripe.com/subscriptions/billing-cycle).
    /// The anchor is the reference point that aligns future billing cycle dates.
    /// It sets the day of week for `week` intervals, the day of month for `month` and `year` intervals, and the month of year for `year` intervals.
    pub fn billing_cycle_anchor(
        mut self,
        billing_cycle_anchor: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        self.inner.billing_cycle_anchor = Some(billing_cycle_anchor.into());
        self
    }
    /// Mutually exclusive with billing_cycle_anchor and only valid with monthly and yearly price intervals.
    /// When provided, the billing_cycle_anchor is set to the next occurrence of the day_of_month at the hour, minute, and second UTC.
    pub fn billing_cycle_anchor_config(
        mut self,
        billing_cycle_anchor_config: impl Into<CreateSubscriptionBillingCycleAnchorConfig>,
    ) -> Self {
        self.inner.billing_cycle_anchor_config = Some(billing_cycle_anchor_config.into());
        self
    }
    /// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
    pub fn billing_mode(mut self, billing_mode: impl Into<CreateSubscriptionBillingMode>) -> Self {
        self.inner.billing_mode = Some(billing_mode.into());
        self
    }
    /// An array of billing schedules, which allow you to bill customers in advance for multiple service periods.
    /// Requires flexible billing mode and API version 2026-05-27.dahlia or later.
    /// Learn more about [prebilling](https://docs.stripe.com/billing/subscriptions/prebilling).
    pub fn billing_schedules(
        mut self,
        billing_schedules: impl Into<Vec<CreateSubscriptionBillingSchedules>>,
    ) -> Self {
        self.inner.billing_schedules = Some(billing_schedules.into());
        self
    }
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    pub fn billing_thresholds(
        mut self,
        billing_thresholds: impl Into<BillingThresholdsParam>,
    ) -> Self {
        self.inner.billing_thresholds = Some(billing_thresholds.into());
        self
    }
    /// A timestamp at which the subscription should cancel.
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    pub fn cancel_at(mut self, cancel_at: impl Into<CreateSubscriptionCancelAt>) -> Self {
        self.inner.cancel_at = Some(cancel_at.into());
        self
    }
    /// Indicate whether this subscription should cancel at the end of the current period (`current_period_end`).
    /// Defaults to `false`.
    pub fn cancel_at_period_end(mut self, cancel_at_period_end: impl Into<bool>) -> Self {
        self.inner.cancel_at_period_end = Some(cancel_at_period_end.into());
        self
    }
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    pub fn collection_method(
        mut self,
        collection_method: impl Into<stripe_shared::CollectionMethod>,
    ) -> Self {
        self.inner.collection_method = Some(collection_method.into());
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// The identifier of the customer to subscribe.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// The identifier of the account representing the customer to subscribe.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
        self
    }
    /// Number of days a customer has to pay invoices generated by this subscription.
    /// Valid only for subscriptions where `collection_method` is set to `send_invoice`.
    pub fn days_until_due(mut self, days_until_due: impl Into<u32>) -> Self {
        self.inner.days_until_due = Some(days_until_due.into());
        self
    }
    /// ID of the default payment method for the subscription.
    /// It must belong to the customer associated with the subscription.
    /// This takes precedence over `default_source`.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://docs.stripe.com/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://docs.stripe.com/api/customers/object#customer_object-default_source).
    pub fn default_payment_method(mut self, default_payment_method: impl Into<String>) -> Self {
        self.inner.default_payment_method = Some(default_payment_method.into());
        self
    }
    /// ID of the default payment source for the subscription.
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If `default_payment_method` is also set, `default_payment_method` will take precedence.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://docs.stripe.com/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://docs.stripe.com/api/customers/object#customer_object-default_source).
    pub fn default_source(mut self, default_source: impl Into<String>) -> Self {
        self.inner.default_source = Some(default_source.into());
        self
    }
    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    pub fn default_tax_rates(mut self, default_tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.default_tax_rates = Some(default_tax_rates.into());
        self
    }
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The coupons to redeem into discounts for the subscription.
    /// If not specified or empty, inherits the discount from the subscription's customer.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// All invoices will be billed using the specified settings.
    pub fn invoice_settings(
        mut self,
        invoice_settings: impl Into<CreateSubscriptionInvoiceSettings>,
    ) -> Self {
        self.inner.invoice_settings = Some(invoice_settings.into());
        self
    }
    /// A list of up to 20 subscription items, each with an attached price.
    pub fn items(mut self, items: impl Into<Vec<CreateSubscriptionItems>>) -> Self {
        self.inner.items = Some(items.into());
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
    /// The account on behalf of which to charge, for each of the subscription's invoices.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// Controls how Stripe handles the first invoice when payment is required and `collection_method=charge_automatically`.
    /// Subscriptions with `collection_method=send_invoice` are automatically activated regardless of the first Invoice status.
    pub fn payment_behavior(
        mut self,
        payment_behavior: impl Into<stripe_shared::PaymentBehavior>,
    ) -> Self {
        self.inner.payment_behavior = Some(payment_behavior.into());
        self
    }
    /// Payment settings to pass to invoices created by the subscription.
    pub fn payment_settings(
        mut self,
        payment_settings: impl Into<CreateSubscriptionPaymentSettings>,
    ) -> Self {
        self.inner.payment_settings = Some(payment_settings.into());
        self
    }
    /// Specifies an interval for how often to bill for any pending invoice items.
    /// It is analogous to calling [Create an invoice](/api/invoices/create) for the given subscription at the specified interval.
    pub fn pending_invoice_item_interval(
        mut self,
        pending_invoice_item_interval: impl Into<PendingInvoiceItemIntervalParams>,
    ) -> Self {
        self.inner.pending_invoice_item_interval = Some(pending_invoice_item_interval.into());
        self
    }
    /// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) resulting from the `billing_cycle_anchor`.
    /// If no value is passed, the default is `create_prorations`.
    pub fn proration_behavior(
        mut self,
        proration_behavior: impl Into<stripe_shared::ProrationBehavior>,
    ) -> Self {
        self.inner.proration_behavior = Some(proration_behavior.into());
        self
    }
    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    pub fn transfer_data(mut self, transfer_data: impl Into<TransferDataSpecs>) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    /// See [Using trial periods on subscriptions](https://docs.stripe.com/billing/subscriptions/trials) to learn more.
    pub fn trial_end(mut self, trial_end: impl Into<CreateSubscriptionTrialEnd>) -> Self {
        self.inner.trial_end = Some(trial_end.into());
        self
    }
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://docs.stripe.com/billing/subscriptions/trials) to learn more.
    pub fn trial_from_plan(mut self, trial_from_plan: impl Into<bool>) -> Self {
        self.inner.trial_from_plan = Some(trial_from_plan.into());
        self
    }
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// See [Using trial periods on subscriptions](https://docs.stripe.com/billing/subscriptions/trials) to learn more.
    pub fn trial_period_days(mut self, trial_period_days: impl Into<u32>) -> Self {
        self.inner.trial_period_days = Some(trial_period_days.into());
        self
    }
    /// Settings related to subscription trials.
    pub fn trial_settings(mut self, trial_settings: impl Into<TrialSettingsConfig>) -> Self {
        self.inner.trial_settings = Some(trial_settings.into());
        self
    }
}
impl Default for CreateSubscription {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateSubscription {
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

impl StripeRequest for CreateSubscription {
    type Output = stripe_shared::Subscription;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/subscriptions").form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct MigrateSubscriptionBuilder {
    billing_mode: MigrateSubscriptionBillingMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MigrateSubscriptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MigrateSubscriptionBuilder").finish_non_exhaustive()
    }
}
impl MigrateSubscriptionBuilder {
    fn new(billing_mode: impl Into<MigrateSubscriptionBillingMode>) -> Self {
        Self { billing_mode: billing_mode.into(), expand: None }
    }
}
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct MigrateSubscriptionBillingMode {
    /// Configure behavior for flexible billing mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible: Option<FlexibleParams>,
    /// Controls the calculation and orchestration of prorations and invoices for subscriptions.
    #[serde(rename = "type")]
    pub type_: MigrateSubscriptionBillingModeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MigrateSubscriptionBillingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MigrateSubscriptionBillingMode").finish_non_exhaustive()
    }
}
impl MigrateSubscriptionBillingMode {
    pub fn new(type_: impl Into<MigrateSubscriptionBillingModeType>) -> Self {
        Self { flexible: None, type_: type_.into() }
    }
}
/// Controls the calculation and orchestration of prorations and invoices for subscriptions.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MigrateSubscriptionBillingModeType {
    Flexible,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MigrateSubscriptionBillingModeType {
    pub fn as_str(&self) -> &str {
        use MigrateSubscriptionBillingModeType::*;
        match self {
            Flexible => "flexible",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MigrateSubscriptionBillingModeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MigrateSubscriptionBillingModeType::*;
        match s {
            "flexible" => Ok(Flexible),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MigrateSubscriptionBillingModeType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MigrateSubscriptionBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MigrateSubscriptionBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MigrateSubscriptionBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MigrateSubscriptionBillingModeType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for MigrateSubscriptionBillingModeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MigrateSubscriptionBillingModeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Upgrade the billing_mode of an existing subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct MigrateSubscription {
    inner: MigrateSubscriptionBuilder,
    subscription: stripe_shared::SubscriptionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MigrateSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MigrateSubscription").finish_non_exhaustive()
    }
}
impl MigrateSubscription {
    /// Construct a new `MigrateSubscription`.
    pub fn new(
        subscription: impl Into<stripe_shared::SubscriptionId>,
        billing_mode: impl Into<MigrateSubscriptionBillingMode>,
    ) -> Self {
        Self {
            subscription: subscription.into(),
            inner: MigrateSubscriptionBuilder::new(billing_mode.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl MigrateSubscription {
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

impl StripeRequest for MigrateSubscription {
    type Output = stripe_shared::Subscription;

    fn build(&self) -> RequestBuilder {
        let subscription = &self.subscription;
        RequestBuilder::new(StripeMethod::Post, format!("/subscriptions/{subscription}/migrate"))
            .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ResumeSubscriptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_cycle_anchor: Option<stripe_shared::BillingCycleAnchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<stripe_shared::ProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_date: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ResumeSubscriptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ResumeSubscriptionBuilder").finish_non_exhaustive()
    }
}
impl ResumeSubscriptionBuilder {
    fn new() -> Self {
        Self {
            billing_cycle_anchor: None,
            expand: None,
            proration_behavior: None,
            proration_date: None,
        }
    }
}
/// Initiates resumption of a paused subscription, optionally resetting the billing cycle anchor and creating prorations.
/// Resume is only available for subscriptions that use `charge_automatically` collection.
/// If Stripe doesn’t generate a resumption invoice, the subscription becomes `active` immediately.
/// When a resumption invoice is generated, Stripe finalizes it immediately.
/// If the invoice is paid or marked uncollectible, the subscription becomes `active`.
/// If the invoice is manually voided, the subscription stays `paused`.
/// If there is no payment attempt within 23 hours, Stripe voids the invoice and the subscription stays `paused`.
/// Learn more about [resuming subscriptions](https://stripe.com/docs/billing/subscriptions/pause#resume-subscriptions).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ResumeSubscription {
    inner: ResumeSubscriptionBuilder,
    subscription: stripe_shared::SubscriptionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ResumeSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ResumeSubscription").finish_non_exhaustive()
    }
}
impl ResumeSubscription {
    /// Construct a new `ResumeSubscription`.
    pub fn new(subscription: impl Into<stripe_shared::SubscriptionId>) -> Self {
        Self { subscription: subscription.into(), inner: ResumeSubscriptionBuilder::new() }
    }
    /// The billing cycle anchor that applies when the subscription is resumed.
    /// Either `now` or `unchanged`.
    /// The default is `now`.
    /// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
    pub fn billing_cycle_anchor(
        mut self,
        billing_cycle_anchor: impl Into<stripe_shared::BillingCycleAnchor>,
    ) -> Self {
        self.inner.billing_cycle_anchor = Some(billing_cycle_anchor.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) resulting from the `billing_cycle_anchor` being `unchanged`.
    /// When the `billing_cycle_anchor` is set to `now` (default value), no prorations are generated.
    /// If no value is passed, the default is `create_prorations`.
    pub fn proration_behavior(
        mut self,
        proration_behavior: impl Into<stripe_shared::ProrationBehavior>,
    ) -> Self {
        self.inner.proration_behavior = Some(proration_behavior.into());
        self
    }
    /// If set, prorations will be calculated as though the subscription was resumed at the given time.
    /// This can be used to apply exactly the same prorations that were previewed with the [create preview](https://stripe.com/docs/api/invoices/create_preview) endpoint.
    pub fn proration_date(mut self, proration_date: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.proration_date = Some(proration_date.into());
        self
    }
}
impl ResumeSubscription {
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

impl StripeRequest for ResumeSubscription {
    type Output = stripe_shared::Subscription;

    fn build(&self) -> RequestBuilder {
        let subscription = &self.subscription;
        RequestBuilder::new(StripeMethod::Post, format!("/subscriptions/{subscription}/resume"))
            .form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateSubscriptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    add_invoice_items: Option<Vec<AddInvoiceItemEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<UpdateSubscriptionAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_cycle_anchor: Option<stripe_shared::BillingCycleAnchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_schedules: Option<Vec<UpdateSubscriptionBillingSchedules>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_thresholds: Option<BillingThresholdsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_at: Option<UpdateSubscriptionCancelAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_at_period_end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancellation_details: Option<CancellationDetailsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_method: Option<stripe_shared::CollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_until_due: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_settings: Option<UpdateSubscriptionInvoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<UpdateSubscriptionItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    off_session: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pause_collection: Option<UpdateSubscriptionPauseCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_behavior: Option<stripe_shared::PaymentBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_settings: Option<UpdateSubscriptionPaymentSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_invoice_item_interval: Option<PendingInvoiceItemIntervalParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<stripe_shared::ProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<TransferDataSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_end: Option<UpdateSubscriptionTrialEnd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_from_plan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_settings: Option<TrialSettingsConfig>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionBuilder").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionBuilder {
    fn new() -> Self {
        Self {
            add_invoice_items: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_schedules: None,
            billing_thresholds: None,
            cancel_at: None,
            cancel_at_period_end: None,
            cancellation_details: None,
            collection_method: None,
            days_until_due: None,
            default_payment_method: None,
            default_source: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            expand: None,
            invoice_settings: None,
            items: None,
            metadata: None,
            off_session: None,
            on_behalf_of: None,
            pause_collection: None,
            payment_behavior: None,
            payment_settings: None,
            pending_invoice_item_interval: None,
            proration_behavior: None,
            proration_date: None,
            transfer_data: None,
            trial_end: None,
            trial_from_plan: None,
            trial_settings: None,
        }
    }
}
/// Automatic tax settings for this subscription.
/// We recommend you only include this parameter when the existing value is being changed.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpdateSubscriptionAutomaticTaxLiability>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionAutomaticTax").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionAutomaticTaxLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionAutomaticTaxLiability").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionAutomaticTaxLiability {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// An array of billing schedules, which allow you to bill customers in advance for multiple service periods.
/// Requires flexible billing mode and API version 2026-05-27.dahlia or later.
/// Learn more about [prebilling](https://docs.stripe.com/billing/subscriptions/prebilling).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionBillingSchedules {
    /// Configure billing schedule differently for individual subscription items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<Vec<BillingSchedulesAppliesTo>>,
    /// The end date for the billing schedule.
    /// You must not set this earlier than current period end for every applicable subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_until: Option<BillingSchedulesBillUntil>,
    /// Specify a key for the billing schedule.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    /// If not provided, a unique key will be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionBillingSchedules {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionBillingSchedules").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionBillingSchedules {
    pub fn new() -> Self {
        Self { applies_to: None, bill_until: None, key: None }
    }
}
impl Default for UpdateSubscriptionBillingSchedules {
    fn default() -> Self {
        Self::new()
    }
}
/// A timestamp at which the subscription should cancel.
/// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
/// If set during a future period, this will always cause a proration for that period.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionCancelAt {
    MaxBilledUntil,
    MaxPeriodEnd,
    MinPeriodEnd,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionCancelAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionCancelAt").finish_non_exhaustive()
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionInvoiceSettings {
    /// The account tax IDs associated with the subscription.
    /// Will be set on invoices generated by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// A list of up to 4 custom fields to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldParams>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Footer to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdateSubscriptionInvoiceSettingsIssuer>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionInvoiceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionInvoiceSettings").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionInvoiceSettings {
    pub fn new() -> Self {
        Self {
            account_tax_ids: None,
            custom_fields: None,
            description: None,
            footer: None,
            issuer: None,
        }
    }
}
impl Default for UpdateSubscriptionInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionInvoiceSettingsIssuer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionInvoiceSettingsIssuer").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionInvoiceSettingsIssuer {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// Delete all usage for a given subscription item.
    /// You must pass this when deleting a usage records subscription item.
    /// `clear_usage` has no effect if the plan has a billing meter attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Subscription item to update.
    /// If you omit `id`, the API adds a new subscription item rather than updating the existing one.
    /// See [Changing a subscription's price](https://docs.stripe.com/billing/subscriptions/change-price#changing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    /// One of `price` or `price_data` is required.
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<RecurringPriceData>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://docs.stripe.com/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionItems").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            clear_usage: None,
            deleted: None,
            discounts: None,
            id: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpdateSubscriptionItems {
    fn default() -> Self {
        Self::new()
    }
}
/// If specified, payment collection for this subscription will be paused.
/// Note that the subscription status will be unchanged and will not be updated to `paused`.
/// Learn more about [pausing collection](https://docs.stripe.com/billing/subscriptions/pause-payment).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPauseCollection {
    /// The payment collection behavior for this subscription while paused.
    pub behavior: stripe_shared::Behavior,
    /// The time after which the subscription will resume collecting payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumes_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPauseCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPauseCollection").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPauseCollection {
    pub fn new(behavior: impl Into<stripe_shared::Behavior>) -> Self {
        Self { behavior: behavior.into(), resumes_at: None }
    }
}
/// Payment settings to pass to invoices created by the subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettings {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptions>,
    /// The list of payment method types (e.g.
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    /// Should not be specified with payment_method_configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<stripe_shared::PaymentMethodTypes>>,
    /// Configure whether Stripe updates `subscription.default_payment_method` when payment succeeds.
    /// Defaults to `off` if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_default_payment_method: Option<stripe_shared::SaveDefaultPaymentMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettings").finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettings {
    pub fn new() -> Self {
        Self {
            payment_method_options: None,
            payment_method_types: None,
            save_default_payment_method: None,
        }
    }
}
impl Default for UpdateSubscriptionPaymentSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment-method-specific configuration to provide to invoices created by the subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<InvoicePaymentMethodOptionsParam>,
    /// This sub-hash contains details about the Billie payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub billie: Option<miniserde::json::Value>,
    /// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance>,
    /// This sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// This sub-hash contains details about the PayTo payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPayto>,
    /// This sub-hash contains details about the Pix payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<SubscriptionPaymentMethodOptionsParam>,
    /// This sub-hash contains details about the SEPA Direct Debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub sepa_debit: Option<miniserde::json::Value>,
    /// This sub-hash contains details about the UPI payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUpi>,
    /// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettingsPaymentMethodOptions")
            .finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptions {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            bancontact: None,
            billie: None,
            card: None,
            customer_balance: None,
            konbini: None,
            payto: None,
            pix: None,
            sepa_debit: None,
            upi: None,
            us_bank_account: None,
        }
    }
}
impl Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<MandateOptionsParam>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit")
            .finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self { mandate_options: None, verification_method: None }
    }
}
impl Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions>,
    /// Selected network to process this Subscription on.
    /// Depends on the available networks of the card attached to the Subscription.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard")
            .finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self { mandate_options: None, network: None, request_three_d_secure: None }
    }
}
impl Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments, specified in the presentment currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_shared::AmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions")
            .finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None }
    }
}
impl Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<BankTransferParam>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance")
            .finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    pub fn new() -> Self {
        Self { bank_transfer: None, funding_type: None }
    }
}
impl Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the PayTo payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPayto {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPayto")
            .finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPayto {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    /// The maximum amount that can be collected in a single invoice.
    /// If you don't specify a maximum, then there is no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The purpose for which payments are made. Has a default value based on your merchant category code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<stripe_shared::Purpose>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions")
            .finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, purpose: None }
    }
}
impl Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the UPI payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUpi {
    /// Configuration options for setting up an eMandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<InvoicePaymentMethodOptionsMandateOptionsParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUpi")
            .finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUpi {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUpi {
    fn default() -> Self {
        Self::new()
    }
}
/// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<InvoiceLinkedAccountOptionsParam>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount")
            .finish_non_exhaustive()
    }
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self { financial_connections: None, verification_method: None }
    }
}
impl Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
/// This will always overwrite any trials that might apply via a subscribed plan.
/// If set, `trial_end` will override the default trial period of the plan the customer is being subscribed to.
/// The `billing_cycle_anchor` will be updated to the `trial_end` value.
/// The special value `now` can be provided to end the customer's trial immediately.
/// Can be at most two years from `billing_cycle_anchor`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscriptionTrialEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscriptionTrialEnd").finish_non_exhaustive()
    }
}
/// Updates an existing subscription to match the specified parameters.
/// When changing prices or quantities, we optionally prorate the price we charge next month to make up for any price changes.
/// To preview how the proration is calculated, use the [create preview](https://stripe.com/docs/api/invoices/create_preview) endpoint.
///
/// By default, we prorate subscription changes.
/// For example, if a customer signs up on May 1 for a $100 price, they’ll be billed $100 immediately.
/// If on May 15 they switch to a $200 price, then on June 1 they’ll be billed $250 ($200 for a renewal of her subscription, plus a $50 prorating adjustment for half of the previous month’s $100 difference).
/// Similarly, a downgrade generates a credit that is applied to the next invoice.
/// We also prorate when you make quantity changes.
/// You can also <a href="/billing/scripts/stripe-authored/proration">use scripts to prorate your billing</a>.
/// To learn more, see <a href="/billing/subscriptions/prorations">Prorations</a>.
///
/// Switching prices does not normally change the billing date or generate an immediate charge unless:
///
/// <ul>
/// <li>The billing interval is changed (for example, from monthly to yearly).</li>
/// <li>The subscription moves from free to paid.</li>
/// <li>A trial starts or ends.</li>
/// </ul>
///
/// In these cases, we apply a credit for the unused time on the previous price, immediately charge the customer using the new price, and reset the billing date.
/// Learn about how [Stripe immediately attempts payment for subscription changes](https://stripe.com/docs/billing/subscriptions/upgrade-downgrade#immediate-payment).
///
/// If you want to charge for an upgrade immediately, pass `proration_behavior` as `always_invoice` to create prorations, automatically invoice the customer for those proration adjustments, and attempt to collect payment.
/// If you pass `create_prorations`, the prorations are created but not automatically invoiced.
/// If you want to bill the customer for the prorations before the subscription’s renewal date, you need to manually [invoice the customer](https://stripe.com/docs/api/invoices/create).
///
/// If you don’t want to prorate, set the `proration_behavior` option to `none`.
/// With this option, the customer is billed $100 on May 1 and $200 on June 1.
/// Similarly, if you set `proration_behavior` to `none` when switching between different billing intervals (for example, from monthly to yearly), we don’t generate any credits for the old subscription’s unused time.
/// We still reset the billing date and bill immediately for the new subscription.
///
/// Updating the quantity on a subscription many times in an hour may result in [rate limiting](https://stripe.com/docs/rate-limits).
/// If you need to bill for a frequently changing quantity, consider integrating [usage-based billing](https://stripe.com/docs/billing/subscriptions/usage-based) instead.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSubscription {
    inner: UpdateSubscriptionBuilder,
    subscription_exposed_id: stripe_shared::SubscriptionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSubscription").finish_non_exhaustive()
    }
}
impl UpdateSubscription {
    /// Construct a new `UpdateSubscription`.
    pub fn new(subscription_exposed_id: impl Into<stripe_shared::SubscriptionId>) -> Self {
        Self {
            subscription_exposed_id: subscription_exposed_id.into(),
            inner: UpdateSubscriptionBuilder::new(),
        }
    }
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
    /// You may pass up to 20 items.
    pub fn add_invoice_items(
        mut self,
        add_invoice_items: impl Into<Vec<AddInvoiceItemEntry>>,
    ) -> Self {
        self.inner.add_invoice_items = Some(add_invoice_items.into());
        self
    }
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    pub fn application_fee_percent(mut self, application_fee_percent: impl Into<f64>) -> Self {
        self.inner.application_fee_percent = Some(application_fee_percent.into());
        self
    }
    /// Automatic tax settings for this subscription.
    /// We recommend you only include this parameter when the existing value is being changed.
    pub fn automatic_tax(
        mut self,
        automatic_tax: impl Into<UpdateSubscriptionAutomaticTax>,
    ) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// Either `now` or `unchanged`.
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
    pub fn billing_cycle_anchor(
        mut self,
        billing_cycle_anchor: impl Into<stripe_shared::BillingCycleAnchor>,
    ) -> Self {
        self.inner.billing_cycle_anchor = Some(billing_cycle_anchor.into());
        self
    }
    /// An array of billing schedules, which allow you to bill customers in advance for multiple service periods.
    /// Requires flexible billing mode and API version 2026-05-27.dahlia or later.
    /// Learn more about [prebilling](https://docs.stripe.com/billing/subscriptions/prebilling).
    pub fn billing_schedules(
        mut self,
        billing_schedules: impl Into<Vec<UpdateSubscriptionBillingSchedules>>,
    ) -> Self {
        self.inner.billing_schedules = Some(billing_schedules.into());
        self
    }
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    pub fn billing_thresholds(
        mut self,
        billing_thresholds: impl Into<BillingThresholdsParam>,
    ) -> Self {
        self.inner.billing_thresholds = Some(billing_thresholds.into());
        self
    }
    /// A timestamp at which the subscription should cancel.
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    pub fn cancel_at(mut self, cancel_at: impl Into<UpdateSubscriptionCancelAt>) -> Self {
        self.inner.cancel_at = Some(cancel_at.into());
        self
    }
    /// Indicate whether this subscription should cancel at the end of the current period (`current_period_end`).
    pub fn cancel_at_period_end(mut self, cancel_at_period_end: impl Into<bool>) -> Self {
        self.inner.cancel_at_period_end = Some(cancel_at_period_end.into());
        self
    }
    /// Details about why this subscription was cancelled
    pub fn cancellation_details(
        mut self,
        cancellation_details: impl Into<CancellationDetailsParam>,
    ) -> Self {
        self.inner.cancellation_details = Some(cancellation_details.into());
        self
    }
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    pub fn collection_method(
        mut self,
        collection_method: impl Into<stripe_shared::CollectionMethod>,
    ) -> Self {
        self.inner.collection_method = Some(collection_method.into());
        self
    }
    /// Number of days a customer has to pay invoices generated by this subscription.
    /// Valid only for subscriptions where `collection_method` is set to `send_invoice`.
    pub fn days_until_due(mut self, days_until_due: impl Into<u32>) -> Self {
        self.inner.days_until_due = Some(days_until_due.into());
        self
    }
    /// ID of the default payment method for the subscription.
    /// It must belong to the customer associated with the subscription.
    /// This takes precedence over `default_source`.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://docs.stripe.com/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://docs.stripe.com/api/customers/object#customer_object-default_source).
    pub fn default_payment_method(mut self, default_payment_method: impl Into<String>) -> Self {
        self.inner.default_payment_method = Some(default_payment_method.into());
        self
    }
    /// ID of the default payment source for the subscription.
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If `default_payment_method` is also set, `default_payment_method` will take precedence.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://docs.stripe.com/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://docs.stripe.com/api/customers/object#customer_object-default_source).
    pub fn default_source(mut self, default_source: impl Into<String>) -> Self {
        self.inner.default_source = Some(default_source.into());
        self
    }
    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    /// Pass an empty string to remove previously-defined tax rates.
    pub fn default_tax_rates(mut self, default_tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.default_tax_rates = Some(default_tax_rates.into());
        self
    }
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The coupons to redeem into discounts for the subscription.
    /// A populated array overwrites the existing discounts on the subscription.
    /// If not specified or empty array, it leaves the subscription's discounts unchanged.
    /// If empty string, it clears the subscription's discounts.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// All invoices will be billed using the specified settings.
    pub fn invoice_settings(
        mut self,
        invoice_settings: impl Into<UpdateSubscriptionInvoiceSettings>,
    ) -> Self {
        self.inner.invoice_settings = Some(invoice_settings.into());
        self
    }
    /// A list of up to 20 subscription items, each with an attached price.
    pub fn items(mut self, items: impl Into<Vec<UpdateSubscriptionItems>>) -> Self {
        self.inner.items = Some(items.into());
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
    /// The account on behalf of which to charge, for each of the subscription's invoices.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// If specified, payment collection for this subscription will be paused.
    /// Note that the subscription status will be unchanged and will not be updated to `paused`.
    /// Learn more about [pausing collection](https://docs.stripe.com/billing/subscriptions/pause-payment).
    pub fn pause_collection(
        mut self,
        pause_collection: impl Into<UpdateSubscriptionPauseCollection>,
    ) -> Self {
        self.inner.pause_collection = Some(pause_collection.into());
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
    /// Payment settings to pass to invoices created by the subscription.
    pub fn payment_settings(
        mut self,
        payment_settings: impl Into<UpdateSubscriptionPaymentSettings>,
    ) -> Self {
        self.inner.payment_settings = Some(payment_settings.into());
        self
    }
    /// Specifies an interval for how often to bill for any pending invoice items.
    /// It is analogous to calling [Create an invoice](/api/invoices/create) for the given subscription at the specified interval.
    pub fn pending_invoice_item_interval(
        mut self,
        pending_invoice_item_interval: impl Into<PendingInvoiceItemIntervalParams>,
    ) -> Self {
        self.inner.pending_invoice_item_interval = Some(pending_invoice_item_interval.into());
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
    /// If set, prorations will be calculated as though the subscription was updated at the given time.
    /// This can be used to apply exactly the same prorations that were previewed with the [create preview](https://stripe.com/docs/api/invoices/create_preview) endpoint.
    /// `proration_date` can also be used to implement custom proration logic, such as prorating by day instead of by second, by providing the time that you wish to use for proration calculations.
    pub fn proration_date(mut self, proration_date: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.proration_date = Some(proration_date.into());
        self
    }
    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    /// This will be unset if you POST an empty value.
    pub fn transfer_data(mut self, transfer_data: impl Into<TransferDataSpecs>) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// If set, `trial_end` will override the default trial period of the plan the customer is being subscribed to.
    /// The `billing_cycle_anchor` will be updated to the `trial_end` value.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    pub fn trial_end(mut self, trial_end: impl Into<UpdateSubscriptionTrialEnd>) -> Self {
        self.inner.trial_end = Some(trial_end.into());
        self
    }
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://docs.stripe.com/billing/subscriptions/trials) to learn more.
    pub fn trial_from_plan(mut self, trial_from_plan: impl Into<bool>) -> Self {
        self.inner.trial_from_plan = Some(trial_from_plan.into());
        self
    }
    /// Settings related to subscription trials.
    pub fn trial_settings(mut self, trial_settings: impl Into<TrialSettingsConfig>) -> Self {
        self.inner.trial_settings = Some(trial_settings.into());
        self
    }
}
impl UpdateSubscription {
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

impl StripeRequest for UpdateSubscription {
    type Output = stripe_shared::Subscription;

    fn build(&self) -> RequestBuilder {
        let subscription_exposed_id = &self.subscription_exposed_id;
        RequestBuilder::new(StripeMethod::Post, format!("/subscriptions/{subscription_exposed_id}"))
            .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CancellationDetailsParam {
    /// Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<stripe_shared::CancellationDetailsFeedback>,
    /// Customized feedback options that provide deeper insight into why the subscription was canceled, if the subscription was canceled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_option: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancellationDetailsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancellationDetailsParam").finish_non_exhaustive()
    }
}
impl CancellationDetailsParam {
    pub fn new() -> Self {
        Self { comment: None, feedback: None, feedback_option: None }
    }
}
impl Default for CancellationDetailsParam {
    fn default() -> Self {
        Self::new()
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
pub struct InvoiceItemPeriodEnd {
    /// A precise Unix timestamp for the end of the invoice item period.
    /// Must be greater than or equal to `period.start`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<stripe_types::Timestamp>,
    /// Select how to calculate the end of the invoice item period.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateSubscriptionAddInvoiceItemsPeriodEndType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceItemPeriodEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceItemPeriodEnd").finish_non_exhaustive()
    }
}
impl InvoiceItemPeriodEnd {
    pub fn new(
        type_: impl Into<stripe_shared::CreateSubscriptionAddInvoiceItemsPeriodEndType>,
    ) -> Self {
        Self { timestamp: None, type_: type_.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct InvoiceItemPeriodStart {
    /// A precise Unix timestamp for the start of the invoice item period.
    /// Must be less than or equal to `period.end`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<stripe_types::Timestamp>,
    /// Select how to calculate the start of the invoice item period.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateSubscriptionAddInvoiceItemsPeriodStartType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceItemPeriodStart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceItemPeriodStart").finish_non_exhaustive()
    }
}
impl InvoiceItemPeriodStart {
    pub fn new(
        type_: impl Into<stripe_shared::CreateSubscriptionAddInvoiceItemsPeriodStartType>,
    ) -> Self {
        Self { timestamp: None, type_: type_.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct OneTimePriceDataWithNegativeAmounts {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) will belong to.
    pub product: String,
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge or a negative integer representing the amount to credit to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OneTimePriceDataWithNegativeAmounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OneTimePriceDataWithNegativeAmounts").finish_non_exhaustive()
    }
}
impl OneTimePriceDataWithNegativeAmounts {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FlexibleParams {
    /// Controls how invoices and invoice items display proration amounts and discount amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_discounts: Option<stripe_shared::ProrationDiscounts>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FlexibleParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FlexibleParams").finish_non_exhaustive()
    }
}
impl FlexibleParams {
    pub fn new() -> Self {
        Self { proration_discounts: None }
    }
}
impl Default for FlexibleParams {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BillingSchedulesAppliesTo {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Controls which subscription items the billing schedule applies to.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateSubscriptionBillingSchedulesAppliesToType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingSchedulesAppliesTo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingSchedulesAppliesTo").finish_non_exhaustive()
    }
}
impl BillingSchedulesAppliesTo {
    pub fn new(
        type_: impl Into<stripe_shared::CreateSubscriptionBillingSchedulesAppliesToType>,
    ) -> Self {
        Self { price: None, type_: type_.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BillingSchedulesBillUntilDuration {
    /// Specifies billing duration. Either `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The multiplier applied to the interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingSchedulesBillUntilDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingSchedulesBillUntilDuration").finish_non_exhaustive()
    }
}
impl BillingSchedulesBillUntilDuration {
    pub fn new(interval: impl Into<stripe_shared::PlanInterval>) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BillingThresholdsParam {
    /// Monetary threshold that triggers the subscription to advance to a new billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingThresholdsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingThresholdsParam").finish_non_exhaustive()
    }
}
impl BillingThresholdsParam {
    pub fn new() -> Self {
        Self { amount_gte: None, reset_billing_cycle_anchor: None }
    }
}
impl Default for BillingThresholdsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CustomFieldParams {
    /// The name of the custom field. This may be up to 40 characters.
    pub name: String,
    /// The value of the custom field. This may be up to 140 characters.
    pub value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomFieldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomFieldParams").finish_non_exhaustive()
    }
}
impl CustomFieldParams {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into() }
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
pub struct MandateOptionsParam {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<stripe_shared::TransactionType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandateOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MandateOptionsParam").finish_non_exhaustive()
    }
}
impl MandateOptionsParam {
    pub fn new() -> Self {
        Self { transaction_type: None }
    }
}
impl Default for MandateOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct InvoicePaymentMethodOptionsParam {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<stripe_shared::PreferredLanguage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsParam").finish_non_exhaustive()
    }
}
impl InvoicePaymentMethodOptionsParam {
    pub fn new() -> Self {
        Self { preferred_language: None }
    }
}
impl Default for InvoicePaymentMethodOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct EuBankTransferParam {
    /// The desired country code of the bank account information.
    /// Permitted values include: `DE`, `FR`, `IE`, or `NL`.
    pub country: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EuBankTransferParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EuBankTransferParam").finish_non_exhaustive()
    }
}
impl EuBankTransferParam {
    pub fn new(country: impl Into<String>) -> Self {
        Self { country: country.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SubscriptionPaymentMethodOptionsMandateOptionsParam {
    /// Amount to be charged for future payments. If not provided, defaults to 40000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Determines if the amount includes the IOF tax. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof: Option<stripe_shared::MandatePixAmountIncludesIof>,
    /// Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`.
    /// If not provided, the mandate will be active until canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Schedule at which the future payments will be charged.
    /// Defaults to the subscription servicing interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<stripe_shared::MandatePixPaymentSchedule>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsMandateOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionPaymentMethodOptionsMandateOptionsParam")
            .finish_non_exhaustive()
    }
}
impl SubscriptionPaymentMethodOptionsMandateOptionsParam {
    pub fn new() -> Self {
        Self { amount: None, amount_includes_iof: None, end_date: None, payment_schedule: None }
    }
}
impl Default for SubscriptionPaymentMethodOptionsMandateOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct InvoicePaymentMethodOptionsMandateOptionsParam {
    /// Amount to be charged for future payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_shared::AmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// End date of the mandate or subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsMandateOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsMandateOptionsParam").finish_non_exhaustive()
    }
}
impl InvoicePaymentMethodOptionsMandateOptionsParam {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None, end_date: None }
    }
}
impl Default for InvoicePaymentMethodOptionsMandateOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct InvoiceLinkedAccountOptionsFiltersParam {
    /// The account subcategories to use to filter for selectable accounts.
    /// Valid subcategories are `checking` and `savings`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_subcategories: Option<Vec<stripe_shared::PaymentMethodUsBankAccountAccountType>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceLinkedAccountOptionsFiltersParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceLinkedAccountOptionsFiltersParam").finish_non_exhaustive()
    }
}
impl InvoiceLinkedAccountOptionsFiltersParam {
    pub fn new() -> Self {
        Self { account_subcategories: None }
    }
}
impl Default for InvoiceLinkedAccountOptionsFiltersParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PendingInvoiceItemIntervalParams {
    /// Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals between invoices.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PendingInvoiceItemIntervalParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PendingInvoiceItemIntervalParams").finish_non_exhaustive()
    }
}
impl PendingInvoiceItemIntervalParams {
    pub fn new(interval: impl Into<stripe_shared::PlanInterval>) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct TransferDataSpecs {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TransferDataSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TransferDataSpecs").finish_non_exhaustive()
    }
}
impl TransferDataSpecs {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount_percent: None, destination: destination.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct EndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: stripe_shared::MissingPaymentMethod,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EndBehavior").finish_non_exhaustive()
    }
}
impl EndBehavior {
    pub fn new(missing_payment_method: impl Into<stripe_shared::MissingPaymentMethod>) -> Self {
        Self { missing_payment_method: missing_payment_method.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct InvoiceItemPeriod {
    /// End of the invoice item period.
    pub end: InvoiceItemPeriodEnd,
    /// Start of the invoice item period.
    pub start: InvoiceItemPeriodStart,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceItemPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceItemPeriod").finish_non_exhaustive()
    }
}
impl InvoiceItemPeriod {
    pub fn new(
        end: impl Into<InvoiceItemPeriodEnd>,
        start: impl Into<InvoiceItemPeriodStart>,
    ) -> Self {
        Self { end: end.into(), start: start.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BillingSchedulesBillUntil {
    /// Specifies the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<BillingSchedulesBillUntilDuration>,
    /// The end date of the billing schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<stripe_types::Timestamp>,
    /// Describes how the billing schedule will determine the end date. Either `duration` or `timestamp`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateSubscriptionBillingSchedulesBillUntilType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingSchedulesBillUntil {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingSchedulesBillUntil").finish_non_exhaustive()
    }
}
impl BillingSchedulesBillUntil {
    pub fn new(
        type_: impl Into<stripe_shared::CreateSubscriptionBillingSchedulesBillUntilType>,
    ) -> Self {
        Self { duration: None, timestamp: None, type_: type_.into() }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BankTransferParam {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<EuBankTransferParam>,
    /// The bank transfer type that can be used for funding.
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankTransferParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankTransferParam").finish_non_exhaustive()
    }
}
impl BankTransferParam {
    pub fn new() -> Self {
        Self { eu_bank_transfer: None, type_: None }
    }
}
impl Default for BankTransferParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SubscriptionPaymentMethodOptionsParam {
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    /// Defaults to 86400 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    /// Configuration options for setting up a mandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SubscriptionPaymentMethodOptionsMandateOptionsParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionPaymentMethodOptionsParam").finish_non_exhaustive()
    }
}
impl SubscriptionPaymentMethodOptionsParam {
    pub fn new() -> Self {
        Self { expires_after_seconds: None, mandate_options: None }
    }
}
impl Default for SubscriptionPaymentMethodOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct InvoiceLinkedAccountOptionsParam {
    /// Provide filters for the linked accounts that the customer can select for the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<InvoiceLinkedAccountOptionsFiltersParam>,
    /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<stripe_shared::Permissions>>,
    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<stripe_shared::Prefetch>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceLinkedAccountOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceLinkedAccountOptionsParam").finish_non_exhaustive()
    }
}
impl InvoiceLinkedAccountOptionsParam {
    pub fn new() -> Self {
        Self { filters: None, permissions: None, prefetch: None }
    }
}
impl Default for InvoiceLinkedAccountOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct TrialSettingsConfig {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: EndBehavior,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TrialSettingsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TrialSettingsConfig").finish_non_exhaustive()
    }
}
impl TrialSettingsConfig {
    pub fn new(end_behavior: impl Into<EndBehavior>) -> Self {
        Self { end_behavior: end_behavior.into() }
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AddInvoiceItemEntry {
    /// Controls whether discounts apply to this invoice item. Defaults to true if no value is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    /// The coupons to redeem into discounts for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The period associated with this invoice item.
    /// If not set, `period.start.type` defaults to `max_item_period_start` and `period.end.type` defaults to `min_item_period_end`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<InvoiceItemPeriod>,
    /// The ID of the price object. One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<OneTimePriceDataWithNegativeAmounts>,
    /// Quantity for this item. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AddInvoiceItemEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AddInvoiceItemEntry").finish_non_exhaustive()
    }
}
impl AddInvoiceItemEntry {
    pub fn new() -> Self {
        Self {
            discountable: None,
            discounts: None,
            metadata: None,
            period: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for AddInvoiceItemEntry {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
pub type CancelSubscriptionCancellationDetailsFeedback = stripe_shared::CancellationDetailsFeedback;
/// The collection method of the subscriptions to retrieve.
/// Either `charge_automatically` or `send_invoice`.
pub type ListSubscriptionCollectionMethod = stripe_shared::CollectionMethod;
/// Select how to calculate the end of the invoice item period.
pub type CreateSubscriptionAddInvoiceItemsPeriodEndType =
    stripe_shared::CreateSubscriptionAddInvoiceItemsPeriodEndType;
/// Select how to calculate the start of the invoice item period.
pub type CreateSubscriptionAddInvoiceItemsPeriodStartType =
    stripe_shared::CreateSubscriptionAddInvoiceItemsPeriodStartType;
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior = stripe_shared::PriceTaxBehavior;
/// Type of the account referenced in the request.
pub type CreateSubscriptionAutomaticTaxLiabilityType = stripe_shared::CreateInvoiceIssuerType;
/// Controls how invoices and invoice items display proration amounts and discount amounts.
pub type CreateSubscriptionBillingModeFlexibleProrationDiscounts =
    stripe_shared::ProrationDiscounts;
/// Controls the calculation and orchestration of prorations and invoices for subscriptions.
/// If no value is passed, the default is `flexible`.
pub type CreateSubscriptionBillingModeType = stripe_shared::CreateSubscriptionBillingModeType;
/// Controls which subscription items the billing schedule applies to.
pub type CreateSubscriptionBillingSchedulesAppliesToType =
    stripe_shared::CreateSubscriptionBillingSchedulesAppliesToType;
/// Specifies billing duration. Either `day`, `week`, `month` or `year`.
pub type CreateSubscriptionBillingSchedulesBillUntilDurationInterval = stripe_shared::PlanInterval;
/// Describes how the billing schedule will determine the end date. Either `duration` or `timestamp`.
pub type CreateSubscriptionBillingSchedulesBillUntilType =
    stripe_shared::CreateSubscriptionBillingSchedulesBillUntilType;
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically`.
pub type CreateSubscriptionCollectionMethod = stripe_shared::CollectionMethod;
/// Type of the account referenced in the request.
pub type CreateSubscriptionInvoiceSettingsIssuerType = stripe_shared::CreateInvoiceIssuerType;
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
pub type CreateSubscriptionItemsPriceDataRecurringInterval = stripe_shared::PlanInterval;
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type CreateSubscriptionItemsPriceDataTaxBehavior = stripe_shared::PriceTaxBehavior;
/// Controls how Stripe handles the first invoice when payment is required and `collection_method=charge_automatically`.
/// Subscriptions with `collection_method=send_invoice` are automatically activated regardless of the first Invoice status.
pub type CreateSubscriptionPaymentBehavior = stripe_shared::PaymentBehavior;
/// Transaction type of the mandate.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType =
    stripe_shared::TransactionType;
/// Verification method for the intent
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_shared::VerificationMethod;
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage =
    stripe_shared::PreferredLanguage;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType =
    stripe_shared::AmountType;
/// Selected network to process this Subscription on.
/// Depends on the available networks of the card attached to the Subscription.
/// Can be only set confirm-time.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork =
    stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork;
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_shared::RequestThreeDSecure;
/// The purpose for which payments are made. Has a default value based on your merchant category code.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose =
    stripe_shared::Purpose;
/// Determines if the amount includes the IOF tax. Defaults to `never`.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof =
    stripe_shared::MandatePixAmountIncludesIof;
/// Schedule at which the future payments will be charged.
/// Defaults to the subscription servicing interval.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule =
    stripe_shared::MandatePixPaymentSchedule;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_shared::AmountType;
/// The account subcategories to use to filter for selectable accounts.
/// Valid subcategories are `checking` and `savings`.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategories =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions =
    stripe_shared::Permissions;
/// List of data features that you would like to retrieve upon account creation.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch =
    stripe_shared::Prefetch;
/// Verification method for the intent
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_shared::VerificationMethod;
/// The list of payment method types (e.g.
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
/// Should not be specified with payment_method_configuration.
pub type CreateSubscriptionPaymentSettingsPaymentMethodTypes = stripe_shared::PaymentMethodTypes;
/// Configure whether Stripe updates `subscription.default_payment_method` when payment succeeds.
/// Defaults to `off` if unspecified.
pub type CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod =
    stripe_shared::SaveDefaultPaymentMethod;
/// Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
pub type CreateSubscriptionPendingInvoiceItemIntervalInterval = stripe_shared::PlanInterval;
/// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) resulting from the `billing_cycle_anchor`.
/// If no value is passed, the default is `create_prorations`.
pub type CreateSubscriptionProrationBehavior = stripe_shared::ProrationBehavior;
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
pub type CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod =
    stripe_shared::MissingPaymentMethod;
/// Controls how invoices and invoice items display proration amounts and discount amounts.
pub type MigrateSubscriptionBillingModeFlexibleProrationDiscounts =
    stripe_shared::ProrationDiscounts;
/// The billing cycle anchor that applies when the subscription is resumed.
/// Either `now` or `unchanged`.
/// The default is `now`.
/// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
pub type ResumeSubscriptionBillingCycleAnchor = stripe_shared::BillingCycleAnchor;
/// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) resulting from the `billing_cycle_anchor` being `unchanged`.
/// When the `billing_cycle_anchor` is set to `now` (default value), no prorations are generated.
/// If no value is passed, the default is `create_prorations`.
pub type ResumeSubscriptionProrationBehavior = stripe_shared::ProrationBehavior;
/// Select how to calculate the end of the invoice item period.
pub type UpdateSubscriptionAddInvoiceItemsPeriodEndType =
    stripe_shared::CreateSubscriptionAddInvoiceItemsPeriodEndType;
/// Select how to calculate the start of the invoice item period.
pub type UpdateSubscriptionAddInvoiceItemsPeriodStartType =
    stripe_shared::CreateSubscriptionAddInvoiceItemsPeriodStartType;
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior = stripe_shared::PriceTaxBehavior;
/// Type of the account referenced in the request.
pub type UpdateSubscriptionAutomaticTaxLiabilityType = stripe_shared::CreateInvoiceIssuerType;
/// Either `now` or `unchanged`.
/// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
/// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
pub type UpdateSubscriptionBillingCycleAnchor = stripe_shared::BillingCycleAnchor;
/// Controls which subscription items the billing schedule applies to.
pub type UpdateSubscriptionBillingSchedulesAppliesToType =
    stripe_shared::CreateSubscriptionBillingSchedulesAppliesToType;
/// Specifies billing duration. Either `day`, `week`, `month` or `year`.
pub type UpdateSubscriptionBillingSchedulesBillUntilDurationInterval = stripe_shared::PlanInterval;
/// Describes how the billing schedule will determine the end date. Either `duration` or `timestamp`.
pub type UpdateSubscriptionBillingSchedulesBillUntilType =
    stripe_shared::CreateSubscriptionBillingSchedulesBillUntilType;
/// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
pub type UpdateSubscriptionCancellationDetailsFeedback = stripe_shared::CancellationDetailsFeedback;
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically`.
pub type UpdateSubscriptionCollectionMethod = stripe_shared::CollectionMethod;
/// Type of the account referenced in the request.
pub type UpdateSubscriptionInvoiceSettingsIssuerType = stripe_shared::CreateInvoiceIssuerType;
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
pub type UpdateSubscriptionItemsPriceDataRecurringInterval = stripe_shared::PlanInterval;
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type UpdateSubscriptionItemsPriceDataTaxBehavior = stripe_shared::PriceTaxBehavior;
/// The payment collection behavior for this subscription while paused.
pub type UpdateSubscriptionPauseCollectionBehavior = stripe_shared::Behavior;
/// Controls how Stripe handles payment when a subscription update requires payment and `collection_method=charge_automatically`.
pub type UpdateSubscriptionPaymentBehavior = stripe_shared::PaymentBehavior;
/// Transaction type of the mandate.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType =
    stripe_shared::TransactionType;
/// Verification method for the intent
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_shared::VerificationMethod;
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage =
    stripe_shared::PreferredLanguage;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType =
    stripe_shared::AmountType;
/// Selected network to process this Subscription on.
/// Depends on the available networks of the card attached to the Subscription.
/// Can be only set confirm-time.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork =
    stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork;
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_shared::RequestThreeDSecure;
/// The purpose for which payments are made. Has a default value based on your merchant category code.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose =
    stripe_shared::Purpose;
/// Determines if the amount includes the IOF tax. Defaults to `never`.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof =
    stripe_shared::MandatePixAmountIncludesIof;
/// Schedule at which the future payments will be charged.
/// Defaults to the subscription servicing interval.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule =
    stripe_shared::MandatePixPaymentSchedule;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_shared::AmountType;
/// The account subcategories to use to filter for selectable accounts.
/// Valid subcategories are `checking` and `savings`.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategories =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions =
    stripe_shared::Permissions;
/// List of data features that you would like to retrieve upon account creation.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch =
    stripe_shared::Prefetch;
/// Verification method for the intent
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_shared::VerificationMethod;
/// The list of payment method types (e.g.
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
/// Should not be specified with payment_method_configuration.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodTypes = stripe_shared::PaymentMethodTypes;
/// Configure whether Stripe updates `subscription.default_payment_method` when payment succeeds.
/// Defaults to `off` if unspecified.
pub type UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod =
    stripe_shared::SaveDefaultPaymentMethod;
/// Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
pub type UpdateSubscriptionPendingInvoiceItemIntervalInterval = stripe_shared::PlanInterval;
/// Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
pub type UpdateSubscriptionProrationBehavior = stripe_shared::ProrationBehavior;
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
pub type UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod =
    stripe_shared::MissingPaymentMethod;
/// Details about why this subscription was cancelled
pub type CancelSubscriptionCancellationDetails = CancellationDetailsParam;
/// The coupons to redeem into discounts for the item.
pub type CreateSubscriptionAddInvoiceItemsDiscounts = DiscountsDataParam;
/// End of the invoice item period.
pub type CreateSubscriptionAddInvoiceItemsPeriodEnd = InvoiceItemPeriodEnd;
/// Start of the invoice item period.
pub type CreateSubscriptionAddInvoiceItemsPeriodStart = InvoiceItemPeriodStart;
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
/// One of `price` or `price_data` is required.
pub type CreateSubscriptionAddInvoiceItemsPriceData = OneTimePriceDataWithNegativeAmounts;
/// Configure behavior for flexible billing mode.
pub type CreateSubscriptionBillingModeFlexible = FlexibleParams;
/// Configure billing schedule differently for individual subscription items.
pub type CreateSubscriptionBillingSchedulesAppliesTo = BillingSchedulesAppliesTo;
/// Specifies the billing period.
pub type CreateSubscriptionBillingSchedulesBillUntilDuration = BillingSchedulesBillUntilDuration;
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// When updating, pass an empty string to remove previously-defined thresholds.
pub type CreateSubscriptionBillingThresholds = BillingThresholdsParam;
/// The coupons to redeem into discounts for the subscription.
/// If not specified or empty, inherits the discount from the subscription's customer.
pub type CreateSubscriptionDiscounts = DiscountsDataParam;
/// A list of up to 4 custom fields to be displayed on the invoice.
pub type CreateSubscriptionInvoiceSettingsCustomFields = CustomFieldParams;
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// Pass an empty string to remove previously-defined thresholds.
pub type CreateSubscriptionItemsBillingThresholds = ItemBillingThresholdsParam;
/// The coupons to redeem into discounts for the subscription item.
pub type CreateSubscriptionItemsDiscounts = DiscountsDataParam;
/// The recurring components of a price such as `interval` and `interval_count`.
pub type CreateSubscriptionItemsPriceDataRecurring = RecurringAdhoc;
/// Additional fields for Mandate creation
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions =
    MandateOptionsParam;
/// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact =
    InvoicePaymentMethodOptionsParam;
/// Configuration for eu_bank_transfer funding type.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer =
    EuBankTransferParam;
/// Configuration options for setting up a mandate
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsPixMandateOptions =
    SubscriptionPaymentMethodOptionsMandateOptionsParam;
/// Configuration options for setting up an eMandate
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsUpiMandateOptions =
    InvoicePaymentMethodOptionsMandateOptionsParam;
/// Provide filters for the linked accounts that the customer can select for the payment method.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters =
    InvoiceLinkedAccountOptionsFiltersParam;
/// Specifies an interval for how often to bill for any pending invoice items.
/// It is analogous to calling [Create an invoice](/api/invoices/create) for the given subscription at the specified interval.
pub type CreateSubscriptionPendingInvoiceItemInterval = PendingInvoiceItemIntervalParams;
/// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
pub type CreateSubscriptionTransferData = TransferDataSpecs;
/// Defines how the subscription should behave when the user's free trial ends.
pub type CreateSubscriptionTrialSettingsEndBehavior = EndBehavior;
/// Configure behavior for flexible billing mode.
pub type MigrateSubscriptionBillingModeFlexible = FlexibleParams;
/// The coupons to redeem into discounts for the item.
pub type UpdateSubscriptionAddInvoiceItemsDiscounts = DiscountsDataParam;
/// End of the invoice item period.
pub type UpdateSubscriptionAddInvoiceItemsPeriodEnd = InvoiceItemPeriodEnd;
/// Start of the invoice item period.
pub type UpdateSubscriptionAddInvoiceItemsPeriodStart = InvoiceItemPeriodStart;
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
/// One of `price` or `price_data` is required.
pub type UpdateSubscriptionAddInvoiceItemsPriceData = OneTimePriceDataWithNegativeAmounts;
/// Configure billing schedule differently for individual subscription items.
pub type UpdateSubscriptionBillingSchedulesAppliesTo = BillingSchedulesAppliesTo;
/// Specifies the billing period.
pub type UpdateSubscriptionBillingSchedulesBillUntilDuration = BillingSchedulesBillUntilDuration;
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// When updating, pass an empty string to remove previously-defined thresholds.
pub type UpdateSubscriptionBillingThresholds = BillingThresholdsParam;
/// Details about why this subscription was cancelled
pub type UpdateSubscriptionCancellationDetails = CancellationDetailsParam;
/// The coupons to redeem into discounts for the subscription.
/// A populated array overwrites the existing discounts on the subscription.
/// If not specified or empty array, it leaves the subscription's discounts unchanged.
/// If empty string, it clears the subscription's discounts.
pub type UpdateSubscriptionDiscounts = DiscountsDataParam;
/// A list of up to 4 custom fields to be displayed on the invoice.
pub type UpdateSubscriptionInvoiceSettingsCustomFields = CustomFieldParams;
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// Pass an empty string to remove previously-defined thresholds.
pub type UpdateSubscriptionItemsBillingThresholds = ItemBillingThresholdsParam;
/// The coupons to redeem into discounts for the subscription item.
pub type UpdateSubscriptionItemsDiscounts = DiscountsDataParam;
/// The recurring components of a price such as `interval` and `interval_count`.
pub type UpdateSubscriptionItemsPriceDataRecurring = RecurringAdhoc;
/// Additional fields for Mandate creation
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions =
    MandateOptionsParam;
/// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact =
    InvoicePaymentMethodOptionsParam;
/// Configuration for eu_bank_transfer funding type.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer =
    EuBankTransferParam;
/// Configuration options for setting up a mandate
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPixMandateOptions =
    SubscriptionPaymentMethodOptionsMandateOptionsParam;
/// Configuration options for setting up an eMandate
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUpiMandateOptions =
    InvoicePaymentMethodOptionsMandateOptionsParam;
/// Provide filters for the linked accounts that the customer can select for the payment method.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters =
    InvoiceLinkedAccountOptionsFiltersParam;
/// Specifies an interval for how often to bill for any pending invoice items.
/// It is analogous to calling [Create an invoice](/api/invoices/create) for the given subscription at the specified interval.
pub type UpdateSubscriptionPendingInvoiceItemInterval = PendingInvoiceItemIntervalParams;
/// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
/// This will be unset if you POST an empty value.
pub type UpdateSubscriptionTransferData = TransferDataSpecs;
/// Defines how the subscription should behave when the user's free trial ends.
pub type UpdateSubscriptionTrialSettingsEndBehavior = EndBehavior;
/// The period associated with this invoice item.
/// If not set, `period.start.type` defaults to `max_item_period_start` and `period.end.type` defaults to `min_item_period_end`.
pub type CreateSubscriptionAddInvoiceItemsPeriod = InvoiceItemPeriod;
/// The end date for the billing schedule.
/// You must not set this earlier than current period end for every applicable subscription item.
pub type CreateSubscriptionBillingSchedulesBillUntil = BillingSchedulesBillUntil;
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
pub type CreateSubscriptionItemsPriceData = RecurringPriceData;
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer =
    BankTransferParam;
/// This sub-hash contains details about the Pix payment method options to pass to the invoice’s PaymentIntent.
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsPix =
    SubscriptionPaymentMethodOptionsParam;
/// Additional fields for Financial Connections Session creation
pub type CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections =
    InvoiceLinkedAccountOptionsParam;
/// Settings related to subscription trials.
pub type CreateSubscriptionTrialSettings = TrialSettingsConfig;
/// The period associated with this invoice item.
/// If not set, `period.start.type` defaults to `max_item_period_start` and `period.end.type` defaults to `min_item_period_end`.
pub type UpdateSubscriptionAddInvoiceItemsPeriod = InvoiceItemPeriod;
/// The end date for the billing schedule.
/// You must not set this earlier than current period end for every applicable subscription item.
pub type UpdateSubscriptionBillingSchedulesBillUntil = BillingSchedulesBillUntil;
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
/// One of `price` or `price_data` is required.
pub type UpdateSubscriptionItemsPriceData = RecurringPriceData;
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer =
    BankTransferParam;
/// This sub-hash contains details about the Pix payment method options to pass to the invoice’s PaymentIntent.
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsPix =
    SubscriptionPaymentMethodOptionsParam;
/// Additional fields for Financial Connections Session creation
pub type UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections =
    InvoiceLinkedAccountOptionsParam;
/// Settings related to subscription trials.
pub type UpdateSubscriptionTrialSettings = TrialSettingsConfig;
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
/// You may pass up to 20 items.
pub type CreateSubscriptionAddInvoiceItems = AddInvoiceItemEntry;
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
/// You may pass up to 20 items.
pub type UpdateSubscriptionAddInvoiceItems = AddInvoiceItemEntry;
