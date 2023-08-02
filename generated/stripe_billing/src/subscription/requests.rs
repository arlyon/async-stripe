
/// Search for subscriptions you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
///
/// Under normal operating conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up to an hour behind during outages.
/// Search functionality is not available to merchants in India.
pub fn search(
    client: &stripe::Client,
    params: SearchSubscription,
) -> stripe::Response<SearchReturned> {
    client.get_query("/subscriptions/search", params)
}
/// By default, returns a list of subscriptions that have not been canceled.
///
/// In order to list canceled subscriptions, specify `status=canceled`.
pub fn list(
    client: &stripe::Client,
    params: ListSubscription,
) -> stripe::Response<stripe_types::List<stripe_types::subscription::Subscription>> {
    client.get_query("/subscriptions", params)
}
/// Creates a new subscription on an existing customer.
///
/// Each customer can have up to 500 active or scheduled subscriptions.  When you create a subscription with `collection_method=charge_automatically`, the first invoice is finalized as part of the request. The `payment_behavior` parameter determines the exact behavior of the initial payment.  To start subscriptions where the first invoice always begins in a `draft` status, use [subscription schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules#managing) instead. Schedules provide the flexibility to model more complex billing configurations that change over time.
pub fn create(
    client: &stripe::Client,
    params: CreateSubscription,
) -> stripe::Response<stripe_types::subscription::Subscription> {
    client.send_form("/subscriptions", params, http_types::Method::Post)
}
/// Updates an existing subscription on a customer to match the specified parameters.
///
/// When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes.
/// To preview how the proration will be calculated, use the [upcoming invoice](https://stripe.com/docs/api#upcoming_invoice) endpoint.
pub fn update(
    client: &stripe::Client,
    subscription_exposed_id: &stripe_types::subscription::SubscriptionId,
    params: UpdateSubscription,
) -> stripe::Response<stripe_types::subscription::Subscription> {
    client.send_form(
        &format!(
            "/subscriptions/{subscription_exposed_id}",
            subscription_exposed_id = subscription_exposed_id
        ),
        params,
        http_types::Method::Post,
    )
}
/// Retrieves the subscription with the given ID.
pub fn retrieve(
    client: &stripe::Client,
    subscription_exposed_id: &stripe_types::subscription::SubscriptionId,
    params: RetrieveSubscription,
) -> stripe::Response<stripe_types::subscription::Subscription> {
    client.get_query(
        &format!(
            "/subscriptions/{subscription_exposed_id}",
            subscription_exposed_id = subscription_exposed_id
        ),
        params,
    )
}
/// Cancels a customer’s subscription immediately.
///
/// The customer will not be charged again for the subscription.  Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually [deleted](https://stripe.com/docs/api#delete_invoiceitem).
/// If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period.
/// But if the subscription is set to cancel immediately, pending prorations will be removed.  By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer.
/// This is intended to prevent unexpected payment attempts after the customer has canceled a subscription.
/// However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed.
/// Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.
pub fn cancel(
    client: &stripe::Client,
    subscription_exposed_id: &stripe_types::subscription::SubscriptionId,
    params: CancelSubscription,
) -> stripe::Response<stripe_types::subscription::Subscription> {
    client.send_form(
        &format!(
            "/subscriptions/{subscription_exposed_id}",
            subscription_exposed_id = subscription_exposed_id
        ),
        params,
        http_types::Method::Delete,
    )
}
/// Initiates resumption of a paused subscription, optionally resetting the billing cycle anchor and creating prorations.
///
/// If a resumption invoice is generated, it must be paid or marked uncollectible before the subscription will be unpaused.
/// If payment succeeds the subscription will become `active`, and if payment fails the subscription will be `past_due`.
/// The resumption invoice will void automatically if not paid by the expiration date.
pub fn resume(
    client: &stripe::Client,
    subscription: &stripe_types::subscription::SubscriptionId,
    params: ResumeSubscription,
) -> stripe::Response<stripe_types::subscription::Subscription> {
    client.send_form(
        &format!("/subscriptions/{subscription}/resume", subscription = subscription),
        params,
        http_types::Method::Post,
    )
}
/// Removes the currently applied discount on a subscription.
pub fn delete_discount(
    client: &stripe::Client,
    subscription_exposed_id: &stripe_types::subscription::SubscriptionId,
) -> stripe::Response<stripe_types::discount::DeletedDiscount> {
    client.send(
        &format!(
            "/subscriptions/{subscription_exposed_id}/discount",
            subscription_exposed_id = subscription_exposed_id
        ),
        http_types::Method::Delete,
    )
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchReturned {
    pub data: Vec<stripe_types::subscription::Subscription>,
    pub has_more: bool,
    pub next_page: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SearchReturnedObject,
    /// The total number of objects that match the query, only accurate up to 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    pub url: String,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SearchReturnedObject {
    SearchResult,
}

impl SearchReturnedObject {
    pub fn as_str(self) -> &'static str {
        use SearchReturnedObject::*;
        match self {
            SearchResult => "search_result",
        }
    }
}

impl std::str::FromStr for SearchReturnedObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SearchReturnedObject::*;
        match s {
            "search_result" => Ok(SearchResult),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SearchReturnedObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SearchReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SearchReturnedObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SearchReturnedObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SearchReturnedObject"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SearchSubscription<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for pagination across multiple pages of results.
    ///
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<&'a str>,
    /// The search query string.
    ///
    /// See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for subscriptions](https://stripe.com/docs/search#query-fields-for-subscriptions).
    pub query: &'a str,
}
impl<'a> SearchSubscription<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            expand: Default::default(),
            limit: Default::default(),
            page: Default::default(),
            query,
        }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListSubscription<'a> {
    /// Filter subscriptions by their automatic tax settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<AutomaticTax>,
    /// The collection method of the subscriptions to retrieve.
    ///
    /// Either `charge_automatically` or `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<stripe_types::RangeQueryTs>,
    /// The ID of the customer whose subscriptions will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID of the plan whose subscriptions will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// Filter for subscriptions that contain this recurring price ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// The status of the subscriptions to retrieve.
    ///
    /// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
    /// Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://stripe.com/docs/billing/subscriptions/overview#subscription-statuses).
    /// Passing in a value of `all` will return subscriptions of all statuses.
    /// If no value is supplied, all subscriptions that have not been canceled are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListSubscriptionStatus>,
    /// Filter for subscriptions that are associated with the specified test clock.
    ///
    /// The response will not include subscriptions with test clocks if this and the customer parameter is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,
}
impl<'a> ListSubscription<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The status of the subscriptions to retrieve.
///
/// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
/// Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://stripe.com/docs/billing/subscriptions/overview#subscription-statuses).
/// Passing in a value of `all` will return subscriptions of all statuses.
/// If no value is supplied, all subscriptions that have not been canceled are returned.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
}

impl ListSubscriptionStatus {
    pub fn as_str(self) -> &'static str {
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
        }
    }
}

impl std::str::FromStr for ListSubscriptionStatus {
    type Err = ();
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
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListSubscriptionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListSubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscription<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<&'a [AddInvoiceItemEntry<'a>]>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this subscription.
    ///
    /// We recommend you only include this parameter when the existing value is being changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<AutomaticTax>,
    /// For new subscriptions, a past timestamp to backdate the subscription's start date to.
    ///
    /// If set, the first invoice will contain a proration for the timespan between the start date and the current time.
    /// Can be combined with trials and the billing cycle anchor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backdate_start_date: Option<stripe_types::Timestamp>,
    /// A future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    ///
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// The timestamp is in UTC format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<stripe_types::Timestamp>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholdsParam>,
    /// A timestamp at which the subscription should cancel.
    ///
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<stripe_types::Timestamp>,
    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,
    /// The ID of the coupon to apply to this subscription.
    ///
    /// A coupon applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The identifier of the customer to subscribe.
    pub customer: &'a str,
    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// Valid only for subscriptions where `collection_method` is set to `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// This takes precedence over `default_source`.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If `default_payment_method` is also set, `default_payment_method` will take precedence.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,
    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<&'a [CreateSubscriptionItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<bool>,
    /// The account on behalf of which to charge, for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// Only applies to subscriptions with `collection_method=charge_automatically`.
    ///
    /// Use `allow_incomplete` to create subscriptions with `status=incomplete` if the first invoice cannot be paid.
    ///
    /// Creating subscriptions with this status allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
    /// For example, SCA regulation may require 3DS authentication to complete payment.
    /// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
    /// This is the default behavior.  Use `default_incomplete` to create Subscriptions with `status=incomplete` when the first invoice requires payment, otherwise start as active.
    /// Subscriptions transition to `status=active` when successfully confirming the payment intent on the first invoice.
    /// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
    /// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.
    /// If the payment intent is not confirmed within 23 hours subscriptions transition to `status=incomplete_expired`, which is a terminal state.  Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's first invoice cannot be paid.
    /// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not create a subscription and returns an error instead.
    /// This was the default behavior for API versions prior to 2019-03-14.
    /// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.  `pending_if_incomplete` is only used with updates and cannot be passed when creating a subscription.  Subscriptions with `collection_method=send_invoice` are automatically activated regardless of the first invoice status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_behavior: Option<PaymentBehavior>,
    /// Payment settings to pass to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<CreateSubscriptionPaymentSettings<'a>>,
    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<PendingInvoiceItemIntervalParams>,
    /// The API ID of a promotion code to apply to this subscription.
    ///
    /// A promotion code applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) resulting from the `billing_cycle_anchor`.
    ///
    /// If no value is passed, the default is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<ProrationBehavior>,
    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataSpecs<'a>>,
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<TrialEnd>,
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_from_plan: Option<bool>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<TrialSettingsConfig>,
}
impl<'a> CreateSubscription<'a> {
    pub fn new(customer: &'a str) -> Self {
        Self {
            add_invoice_items: Default::default(),
            application_fee_percent: Default::default(),
            automatic_tax: Default::default(),
            backdate_start_date: Default::default(),
            billing_cycle_anchor: Default::default(),
            billing_thresholds: Default::default(),
            cancel_at: Default::default(),
            cancel_at_period_end: Default::default(),
            collection_method: Default::default(),
            coupon: Default::default(),
            currency: Default::default(),
            customer,
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            items: Default::default(),
            metadata: Default::default(),
            off_session: Default::default(),
            on_behalf_of: Default::default(),
            payment_behavior: Default::default(),
            payment_settings: Default::default(),
            pending_invoice_item_interval: Default::default(),
            promotion_code: Default::default(),
            proration_behavior: Default::default(),
            transfer_data: Default::default(),
            trial_end: Default::default(),
            trial_from_plan: Default::default(),
            trial_period_days: Default::default(),
            trial_settings: Default::default(),
        }
    }
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionItems<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<RecurringPriceData<'a>>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateSubscriptionItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment settings to pass to invoices created by the subscription.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettings<'a> {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptions<'a>>,
    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [PaymentMethodTypes]>,
    /// Either `off`, or `on_subscription`.
    ///
    /// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_default_payment_method: Option<SaveDefaultPaymentMethod>,
}
impl<'a> CreateSubscriptionPaymentSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration to provide to invoices created by the subscription.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptions<'a> {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact>,
    /// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard<'a>>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance<'a>>,
    /// This sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<InvoicePaymentMethodOptionsParam>,
    /// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> CreateSubscriptionPaymentSettingsPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<MandateOptionsParam>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<PreferredLanguage>,
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard<'a> {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions<'a>>,
    /// Selected network to process this Subscription on.
    ///
    /// Depends on the available networks of the card attached to the Subscription.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<RequestThreeDSecure>,
}
impl<'a> CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<AmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}
impl<'a> CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<BankTransferParam<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<&'a str>,
}
impl<'a> CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<InvoiceLinkedAccountOptionsParam<'a>>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl<'a> CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscription<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<&'a [AddInvoiceItemEntry<'a>]>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this subscription.
    ///
    /// We recommend you only include this parameter when the existing value is being changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<AutomaticTax>,
    /// Either `now` or `unchanged`.
    ///
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<BillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholdsParam>,
    /// A timestamp at which the subscription should cancel.
    ///
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<stripe_types::Timestamp>,
    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    /// Details about why this subscription was cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_details: Option<CancellationDetailsParam<'a>>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,
    /// The ID of the coupon to apply to this subscription.
    ///
    /// A coupon applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// Valid only for subscriptions where `collection_method` is set to `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// This takes precedence over `default_source`.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If `default_payment_method` is also set, `default_payment_method` will take precedence.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,
    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    /// Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<&'a [UpdateSubscriptionItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<bool>,
    /// The account on behalf of which to charge, for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// If specified, payment collection for this subscription will be paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_collection: Option<UpdateSubscriptionPauseCollection>,
    /// Use `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid.
    ///
    /// This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
    /// For example, SCA regulation may require 3DS authentication to complete payment.
    /// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
    /// This is the default behavior.  Use `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent.
    /// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
    /// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.  Use `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates).
    /// When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).  Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid.
    /// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead.
    /// This was the default behavior for API versions prior to 2019-03-14.
    /// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_behavior: Option<PaymentBehavior>,
    /// Payment settings to pass to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<UpdateSubscriptionPaymentSettings<'a>>,
    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<PendingInvoiceItemIntervalParams>,
    /// The promotion code to apply to this subscription.
    ///
    /// A promotion code applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<ProrationBehavior>,
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply exactly the same proration that was previewed with [upcoming invoice](https://stripe.com/docs/api#upcoming_invoice) endpoint.
    /// It can also be used to implement custom proration logic, such as prorating by day instead of by second, by providing the time that you wish to use for proration calculations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<stripe_types::Timestamp>,
    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    ///
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataSpecs<'a>>,
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<TrialEnd>,
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_from_plan: Option<bool>,
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<TrialSettingsConfig>,
}
impl<'a> UpdateSubscription<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionItems<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// Delete all usage for a given subscription item.
    ///
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// The ID of the price object.
    ///
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<RecurringPriceData<'a>>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpdateSubscriptionItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If specified, payment collection for this subscription will be paused.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionPauseCollection {
    /// The payment collection behavior for this subscription while paused.
    ///
    /// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
    pub behavior: UpdateSubscriptionPauseCollectionBehavior,
    /// The time after which the subscription will resume collecting payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumes_at: Option<stripe_types::Timestamp>,
}
impl UpdateSubscriptionPauseCollection {
    pub fn new(behavior: UpdateSubscriptionPauseCollectionBehavior) -> Self {
        Self { behavior, resumes_at: Default::default() }
    }
}
/// The payment collection behavior for this subscription while paused.
///
/// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateSubscriptionPauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}

impl UpdateSubscriptionPauseCollectionBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPauseCollectionBehavior::*;
        match self {
            KeepAsDraft => "keep_as_draft",
            MarkUncollectible => "mark_uncollectible",
            Void => "void",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionPauseCollectionBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPauseCollectionBehavior::*;
        match s {
            "keep_as_draft" => Ok(KeepAsDraft),
            "mark_uncollectible" => Ok(MarkUncollectible),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPauseCollectionBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateSubscriptionPauseCollectionBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Payment settings to pass to invoices created by the subscription.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettings<'a> {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptions<'a>>,
    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [PaymentMethodTypes]>,
    /// Either `off`, or `on_subscription`.
    ///
    /// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_default_payment_method: Option<SaveDefaultPaymentMethod>,
}
impl<'a> UpdateSubscriptionPaymentSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration to provide to invoices created by the subscription.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptions<'a> {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact>,
    /// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard<'a>>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance<'a>>,
    /// This sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<InvoicePaymentMethodOptionsParam>,
    /// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> UpdateSubscriptionPaymentSettingsPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<MandateOptionsParam>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<PreferredLanguage>,
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard<'a> {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions<'a>>,
    /// Selected network to process this Subscription on.
    ///
    /// Depends on the available networks of the card attached to the Subscription.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<RequestThreeDSecure>,
}
impl<'a> UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<AmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}
impl<'a> UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<BankTransferParam<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<&'a str>,
}
impl<'a> UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<InvoiceLinkedAccountOptionsParam<'a>>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl<'a> UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveSubscription<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSubscription<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelSubscription<'a> {
    /// Details about why this subscription was cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_details: Option<CancellationDetailsParam<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Will generate a final invoice that invoices for any un-invoiced metered usage and new/pending proration invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_now: Option<bool>,
    /// Will generate a proration invoice item that credits remaining unused time until the subscription period end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
}
impl<'a> CancelSubscription<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ResumeSubscription<'a> {
    /// Either `now` or `unchanged`.
    ///
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// Setting the value to `unchanged` advances the subscription's billing cycle anchor to the period that surrounds the current time.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<BillingCycleAnchor>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<ProrationBehavior>,
    /// If set, the proration will be calculated as though the subscription was resumed at the given time.
    ///
    /// This can be used to apply exactly the same proration that was previewed with [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<stripe_types::Timestamp>,
}
impl<'a> ResumeSubscription<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}
impl AutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl CollectionMethod {
    pub fn as_str(self) -> &'static str {
        use CollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for CollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl TaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for TaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct BillingThresholdsParam {
    /// Monetary threshold that triggers the subscription to advance to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    ///
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl BillingThresholdsParam {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ItemBillingThresholdsParam {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}
impl ItemBillingThresholdsParam {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interval {
    Day,
    Month,
    Week,
    Year,
}

impl Interval {
    pub fn as_str(self) -> &'static str {
        use Interval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for Interval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Interval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Interval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl PaymentBehavior {
    pub fn as_str(self) -> &'static str {
        use PaymentBehavior::*;
        match self {
            AllowIncomplete => "allow_incomplete",
            DefaultIncomplete => "default_incomplete",
            ErrorIfIncomplete => "error_if_incomplete",
            PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl std::str::FromStr for PaymentBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentBehavior::*;
        match s {
            "allow_incomplete" => Ok(AllowIncomplete),
            "default_incomplete" => Ok(DefaultIncomplete),
            "error_if_incomplete" => Ok(ErrorIfIncomplete),
            "pending_if_incomplete" => Ok(PendingIfIncomplete),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PaymentBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionType {
    Business,
    Personal,
}

impl TransactionType {
    pub fn as_str(self) -> &'static str {
        use TransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr for TransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl VerificationMethod {
    pub fn as_str(self) -> &'static str {
        use VerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for VerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use VerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl PreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use PreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Fr => "fr",
            Nl => "nl",
        }
    }
}

impl std::str::FromStr for PreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AmountType {
    Fixed,
    Maximum,
}

impl AmountType {
    pub fn as_str(self) -> &'static str {
        use AmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr for AmountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Network {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl Network {
    pub fn as_str(self) -> &'static str {
        use Network::*;
        match self {
            Amex => "amex",
            CartesBancaires => "cartes_bancaires",
            Diners => "diners",
            Discover => "discover",
            EftposAu => "eftpos_au",
            Interac => "interac",
            Jcb => "jcb",
            Mastercard => "mastercard",
            Unionpay => "unionpay",
            Unknown => "unknown",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for Network {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Network::*;
        match s {
            "amex" => Ok(Amex),
            "cartes_bancaires" => Ok(CartesBancaires),
            "diners" => Ok(Diners),
            "discover" => Ok(Discover),
            "eftpos_au" => Ok(EftposAu),
            "interac" => Ok(Interac),
            "jcb" => Ok(Jcb),
            "mastercard" => Ok(Mastercard),
            "unionpay" => Ok(Unionpay),
            "unknown" => Ok(Unknown),
            "visa" => Ok(Visa),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Network {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Network {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestThreeDSecure {
    Any,
    Automatic,
}

impl RequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use RequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
        }
    }
}

impl std::str::FromStr for RequestThreeDSecure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for RequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for RequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct EuBankTransferParam<'a> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> EuBankTransferParam<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct InvoicePaymentMethodOptionsParam {}
impl InvoicePaymentMethodOptionsParam {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Permissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl Permissions {
    pub fn as_str(self) -> &'static str {
        use Permissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for Permissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Permissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Permissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl PaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodTypes::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            AcssDebit => "acss_debit",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Konbini => "konbini",
            Link => "link",
            Paynow => "paynow",
            Paypal => "paypal",
            Promptpay => "promptpay",
            SepaCreditTransfer => "sepa_credit_transfer",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
        }
    }
}

impl std::str::FromStr for PaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodTypes::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "acss_debit" => Ok(AcssDebit),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "promptpay" => Ok(Promptpay),
            "sepa_credit_transfer" => Ok(SepaCreditTransfer),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

impl SaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use SaveDefaultPaymentMethod::*;
        match self {
            Off => "off",
            OnSubscription => "on_subscription",
        }
    }
}

impl std::str::FromStr for SaveDefaultPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SaveDefaultPaymentMethod::*;
        match s {
            "off" => Ok(Off),
            "on_subscription" => Ok(OnSubscription),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SaveDefaultPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SaveDefaultPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl ProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use ProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for ProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct TransferDataSpecs<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> TransferDataSpecs<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount_percent: Default::default(), destination }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum TrialEnd {
    Now,
    Timestamp(stripe_types::Timestamp),
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl MissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use MissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
        }
    }
}

impl std::str::FromStr for MissingPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for MissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for MissingPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BillingCycleAnchor {
    Now,
    Unchanged,
}

impl BillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use BillingCycleAnchor::*;
        match self {
            Now => "now",
            Unchanged => "unchanged",
        }
    }
}

impl std::str::FromStr for BillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCycleAnchor::*;
        match s {
            "now" => Ok(Now),
            "unchanged" => Ok(Unchanged),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Feedback {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl Feedback {
    pub fn as_str(self) -> &'static str {
        use Feedback::*;
        match self {
            CustomerService => "customer_service",
            LowQuality => "low_quality",
            MissingFeatures => "missing_features",
            Other => "other",
            SwitchedService => "switched_service",
            TooComplex => "too_complex",
            TooExpensive => "too_expensive",
            Unused => "unused",
        }
    }
}

impl std::str::FromStr for Feedback {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Feedback::*;
        match s {
            "customer_service" => Ok(CustomerService),
            "low_quality" => Ok(LowQuality),
            "missing_features" => Ok(MissingFeatures),
            "other" => Ok(Other),
            "switched_service" => Ok(SwitchedService),
            "too_complex" => Ok(TooComplex),
            "too_expensive" => Ok(TooExpensive),
            "unused" => Ok(Unused),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Feedback {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Feedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Feedback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct OneTimePriceDataWithNegativeAmounts<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> OneTimePriceDataWithNegativeAmounts<'a> {
    pub fn new(currency: stripe_types::Currency, product: &'a str) -> Self {
        Self {
            currency,
            product,
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RecurringAdhoc {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: Interval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl RecurringAdhoc {
    pub fn new(interval: Interval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct MandateOptionsParam {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
}
impl MandateOptionsParam {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct BankTransferParam<'a> {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<EuBankTransferParam<'a>>,
    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
}
impl<'a> BankTransferParam<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct InvoiceLinkedAccountOptionsParam<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<&'a [Permissions]>,
}
impl<'a> InvoiceLinkedAccountOptionsParam<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PendingInvoiceItemIntervalParams {
    /// Specifies invoicing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: Interval,
    /// The number of intervals between invoices.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl PendingInvoiceItemIntervalParams {
    pub fn new(interval: Interval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct EndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: MissingPaymentMethod,
}
impl EndBehavior {
    pub fn new(missing_payment_method: MissingPaymentMethod) -> Self {
        Self { missing_payment_method }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancellationDetailsParam<'a> {
    /// Additional comments about why the user canceled the subscription, if the subscription was cancelled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<&'a str>,
    /// The customer submitted reason for why they cancelled, if the subscription was cancelled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<Feedback>,
}
impl<'a> CancellationDetailsParam<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct AddInvoiceItemEntry<'a> {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<OneTimePriceDataWithNegativeAmounts<'a>>,
    /// Quantity for this item.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item.
    ///
    /// When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> AddInvoiceItemEntry<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RecurringPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: RecurringAdhoc,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> RecurringPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: RecurringAdhoc,
    ) -> Self {
        Self {
            currency,
            product,
            recurring,
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct TrialSettingsConfig {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: EndBehavior,
}
impl TrialSettingsConfig {
    pub fn new(end_behavior: EndBehavior) -> Self {
        Self { end_behavior }
    }
}
