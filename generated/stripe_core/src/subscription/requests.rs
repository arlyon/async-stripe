use stripe::{Client, Response};

impl stripe_core::subscription::Subscription {
    /// Search for subscriptions you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
    /// Don’t use search in read-after-write flows where strict consistency is necessary.
    ///
    /// Under normal operating conditions, data is searchable in less than a minute.
    /// Occasionally, propagation of new or updated data can be up to an hour behind during outages.
    /// Search functionality is not available to merchants in India.
    pub fn search(client: &Client, params: SearchSubscription) -> Response<SearchReturned> {
        client.get_query("/subscriptions/search", params)
    }
    /// By default, returns a list of subscriptions that have not been canceled.
    ///
    /// In order to list canceled subscriptions, specify `status=canceled`.
    pub fn list(
        client: &Client,
        params: ListSubscription,
    ) -> Response<stripe_types::List<stripe_core::subscription::Subscription>> {
        client.get_query("/subscriptions", params)
    }
    /// Creates a new subscription on an existing customer.
    ///
    /// Each customer can have up to 500 active or scheduled subscriptions.  When you create a subscription with `collection_method=charge_automatically`, the first invoice is finalized as part of the request. The `payment_behavior` parameter determines the exact behavior of the initial payment.  To start subscriptions where the first invoice always begins in a `draft` status, use [subscription schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules#managing) instead. Schedules provide the flexibility to model more complex billing configurations that change over time.
    pub fn create(
        client: &Client,
        params: CreateSubscription,
    ) -> Response<stripe_core::subscription::Subscription> {
        client.send_form("/subscriptions", params, http_types::Method::Post)
    }
    /// Updates an existing subscription on a customer to match the specified parameters.
    ///
    /// When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes.
    /// To preview how the proration will be calculated, use the [upcoming invoice](https://stripe.com/docs/api#upcoming_invoice) endpoint.
    pub fn update(
        client: &Client,
        subscription_exposed_id: &str,
        params: UpdateSubscription,
    ) -> Response<stripe_core::subscription::Subscription> {
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
        client: &Client,
        subscription_exposed_id: &str,
        params: RetrieveSubscription,
    ) -> Response<stripe_core::subscription::Subscription> {
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
        client: &Client,
        subscription_exposed_id: &str,
        params: CancelSubscription,
    ) -> Response<stripe_core::subscription::Subscription> {
        client.send_form(
            &format!(
                "/subscriptions/{subscription_exposed_id}",
                subscription_exposed_id = subscription_exposed_id
            ),
            params,
            http_types::Method::Delete,
        )
    }
    /// Removes the currently applied discount on a subscription.
    pub fn delete_discount(
        client: &Client,
        subscription_exposed_id: &str,
    ) -> Response<stripe_core::discount::DeletedDiscount> {
        client.send(
            &format!(
                "/subscriptions/{subscription_exposed_id}/discount",
                subscription_exposed_id = subscription_exposed_id
            ),
            http_types::Method::Delete,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SearchReturned {
    pub data: Vec<stripe_core::subscription::Subscription>,
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SearchReturned {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SearchReturnedObject {
    SearchResult,
}

impl SearchReturnedObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SearchResult => "search_result",
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
    /// The collection method of the subscriptions to retrieve.
    ///
    /// Either `charge_automatically` or `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<ListSubscriptionCollectionMethod>,
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
/// The collection method of the subscriptions to retrieve.
///
/// Either `charge_automatically` or `send_invoice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListSubscriptionCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl ListSubscriptionCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for ListSubscriptionCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListSubscriptionCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The status of the subscriptions to retrieve.
///
/// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
/// Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://stripe.com/docs/billing/subscriptions/overview#subscription-statuses).
/// Passing in a value of `all` will return subscriptions of all statuses.
/// If no value is supplied, all subscriptions that have not been canceled are returned.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListSubscriptionStatus {
    Active,
    All,
    Canceled,
    Ended,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

impl ListSubscriptionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::All => "all",
            Self::Canceled => "canceled",
            Self::Ended => "ended",
            Self::Incomplete => "incomplete",
            Self::IncompleteExpired => "incomplete_expired",
            Self::PastDue => "past_due",
            Self::Trialing => "trialing",
            Self::Unpaid => "unpaid",
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscription<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<&'a [CreateSubscriptionAddInvoiceItems<'a>]>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this subscription.
    ///
    /// We recommend you only include this parameter when the existing value is being changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateSubscriptionAutomaticTax>,
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
    pub billing_thresholds: Option<CreateSubscriptionBillingThresholds>,
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
    pub collection_method: Option<CreateSubscriptionCollectionMethod>,
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
    pub metadata: Option<&'a stripe_types::Metadata>,
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
    pub payment_behavior: Option<CreateSubscriptionPaymentBehavior>,
    /// Payment settings to pass to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<CreateSubscriptionPaymentSettings<'a>>,
    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<CreateSubscriptionPendingInvoiceItemInterval>,
    /// The API ID of a promotion code to apply to this subscription.
    ///
    /// A promotion code applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) resulting from the `billing_cycle_anchor`.
    ///
    /// If no value is passed, the default is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateSubscriptionProrationBehavior>,
    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateSubscriptionTransferData<'a>>,
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<CreateSubscriptionTrialEnd>,
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
        }
    }
}
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
///
/// You may pass up to 20 items.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionAddInvoiceItems<'a> {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateSubscriptionAddInvoiceItemsPriceData<'a>>,
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
impl<'a> CreateSubscriptionAddInvoiceItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionAddInvoiceItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateSubscriptionAddInvoiceItemsPriceData<'a> {
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
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Automatic tax settings for this subscription.
///
/// We recommend you only include this parameter when the existing value is being changed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}
impl CreateSubscriptionAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
///
/// Pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    ///
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl CreateSubscriptionBillingThresholds {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl CreateSubscriptionCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for CreateSubscriptionCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionItems<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<CreateSubscriptionItemsBillingThresholds>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateSubscriptionItemsPriceData<'a>>,
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
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
///
/// When updating, pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionItemsBillingThresholds {
    /// Usage threshold that triggers the subscription to advance to a new billing period.
    pub usage_gte: i64,
}
impl CreateSubscriptionItemsBillingThresholds {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: CreateSubscriptionItemsPriceDataRecurring,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSubscriptionItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateSubscriptionItemsPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: CreateSubscriptionItemsPriceDataRecurring,
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
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: CreateSubscriptionItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateSubscriptionItemsPriceDataRecurring {
    pub fn new(interval: CreateSubscriptionItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateSubscriptionItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for CreateSubscriptionItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateSubscriptionItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateSubscriptionItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl CreateSubscriptionPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllowIncomplete => "allow_incomplete",
            Self::DefaultIncomplete => "default_incomplete",
            Self::ErrorIfIncomplete => "error_if_incomplete",
            Self::PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
    pub payment_method_types: Option<&'a [CreateSubscriptionPaymentSettingsPaymentMethodTypes]>,
    /// Either `off`, or `on_subscription`.
    ///
    /// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_default_payment_method:
        Option<CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod>,
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
    pub konbini: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini>,
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
    pub mandate_options:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
    pub network: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
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
    pub amount_type:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}
impl<'a> CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Selected network to process this Subscription on.
///
/// Depends on the available networks of the card attached to the Subscription.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amex => "amex",
            Self::CartesBancaires => "cartes_bancaires",
            Self::Diners => "diners",
            Self::Discover => "discover",
            Self::Interac => "interac",
            Self::Jcb => "jcb",
            Self::Mastercard => "mastercard",
            Self::Unionpay => "unionpay",
            Self::Unknown => "unknown",
            Self::Visa => "visa",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<
        CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a>,
    >,
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
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    /// Configuration for eu_bank_transfer funding type.
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>>,
    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
#[serde(skip_serializing_if = "Option::is_none")]
pub type_: Option<&'a str>,

}
impl<'a> CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
    'a,
> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a>
    CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
        'a,
    >
{
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// This sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini {}
impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<
        CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a>,
    >,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<
        CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}
impl<'a> CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<&'a [CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions]>,

}
impl<'a>
    CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a>
{
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
Self::Balances => "balances",
Self::Ownership => "ownership",
Self::PaymentMethod => "payment_method",
Self::Transactions => "transactions",

        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of payment method types (e.g.
///
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    CustomerBalance,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    Paynow,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AchCreditTransfer => "ach_credit_transfer",
            Self::AchDebit => "ach_debit",
            Self::AcssDebit => "acss_debit",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Paynow => "paynow",
            Self::Promptpay => "promptpay",
            Self::SepaCreditTransfer => "sepa_credit_transfer",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Either `off`, or `on_subscription`.
///
/// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

impl CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::OnSubscription => "on_subscription",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies an interval for how often to bill for any pending invoice items.
///
/// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionPendingInvoiceItemInterval {
    /// Specifies invoicing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: CreateSubscriptionPendingInvoiceItemIntervalInterval,
    /// The number of intervals between invoices.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateSubscriptionPendingInvoiceItemInterval {
    pub fn new(interval: CreateSubscriptionPendingInvoiceItemIntervalInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies invoicing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPendingInvoiceItemIntervalInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateSubscriptionPendingInvoiceItemIntervalInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPendingInvoiceItemIntervalInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPendingInvoiceItemIntervalInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) resulting from the `billing_cycle_anchor`.
///
/// If no value is passed, the default is `create_prorations`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl CreateSubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> CreateSubscriptionTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount_percent: Default::default(), destination }
    }
}
/// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
///
/// This will always overwrite any trials that might apply via a subscribed plan.
/// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
/// The special value `now` can be provided to end the customer's trial immediately.
/// Can be at most two years from `billing_cycle_anchor`.
/// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CreateSubscriptionTrialEnd {
    Now,
    Timestamp(stripe_types::Timestamp),
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscription<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<&'a [UpdateSubscriptionAddInvoiceItems<'a>]>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this subscription.
    ///
    /// We recommend you only include this parameter when the existing value is being changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateSubscriptionAutomaticTax>,
    /// Either `now` or `unchanged`.
    ///
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpdateSubscriptionBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpdateSubscriptionBillingThresholds>,
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
    pub collection_method: Option<UpdateSubscriptionCollectionMethod>,
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
    pub metadata: Option<&'a stripe_types::Metadata>,
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
    pub payment_behavior: Option<UpdateSubscriptionPaymentBehavior>,
    /// Payment settings to pass to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<UpdateSubscriptionPaymentSettings<'a>>,
    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<UpdateSubscriptionPendingInvoiceItemInterval>,
    /// The promotion code to apply to this subscription.
    ///
    /// A promotion code applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpdateSubscriptionProrationBehavior>,
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply exactly the same proration that was previewed with [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    /// It can also be used to implement custom proration logic, such as prorating by day instead of by second, by providing the time that you wish to use for proration calculations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<stripe_types::Timestamp>,
    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    ///
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdateSubscriptionTransferData<'a>>,
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<UpdateSubscriptionTrialEnd>,
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_from_plan: Option<bool>,
}
impl<'a> UpdateSubscription<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
///
/// You may pass up to 20 items.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionAddInvoiceItems<'a> {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateSubscriptionAddInvoiceItemsPriceData<'a>>,
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
impl<'a> UpdateSubscriptionAddInvoiceItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionAddInvoiceItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateSubscriptionAddInvoiceItemsPriceData<'a> {
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
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Automatic tax settings for this subscription.
///
/// We recommend you only include this parameter when the existing value is being changed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}
impl UpdateSubscriptionAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Either `now` or `unchanged`.
///
/// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
}

impl UpdateSubscriptionBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
            Self::Unchanged => "unchanged",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
///
/// Pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    ///
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl UpdateSubscriptionBillingThresholds {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl UpdateSubscriptionCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionItems<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpdateSubscriptionItemsBillingThresholds>,
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
    pub metadata: Option<&'a stripe_types::Metadata>,
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
    pub price_data: Option<UpdateSubscriptionItemsPriceData<'a>>,
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
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
///
/// When updating, pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionItemsBillingThresholds {
    /// Usage threshold that triggers the subscription to advance to a new billing period.
    pub usage_gte: i64,
}
impl UpdateSubscriptionItemsBillingThresholds {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpdateSubscriptionItemsPriceDataRecurring,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateSubscriptionItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateSubscriptionItemsPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: UpdateSubscriptionItemsPriceDataRecurring,
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
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: UpdateSubscriptionItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpdateSubscriptionItemsPriceDataRecurring {
    pub fn new(interval: UpdateSubscriptionItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl UpdateSubscriptionItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateSubscriptionItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}

impl UpdateSubscriptionPauseCollectionBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepAsDraft => "keep_as_draft",
            Self::MarkUncollectible => "mark_uncollectible",
            Self::Void => "void",
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl UpdateSubscriptionPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllowIncomplete => "allow_incomplete",
            Self::DefaultIncomplete => "default_incomplete",
            Self::ErrorIfIncomplete => "error_if_incomplete",
            Self::PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
    pub payment_method_types: Option<&'a [UpdateSubscriptionPaymentSettingsPaymentMethodTypes]>,
    /// Either `off`, or `on_subscription`.
    ///
    /// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_default_payment_method:
        Option<UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod>,
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
    pub konbini: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini>,
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
    pub mandate_options:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
    pub network: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
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
    pub amount_type:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}
impl<'a> UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Selected network to process this Subscription on.
///
/// Depends on the available networks of the card attached to the Subscription.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amex => "amex",
            Self::CartesBancaires => "cartes_bancaires",
            Self::Diners => "diners",
            Self::Discover => "discover",
            Self::Interac => "interac",
            Self::Jcb => "jcb",
            Self::Mastercard => "mastercard",
            Self::Unionpay => "unionpay",
            Self::Unknown => "unknown",
            Self::Visa => "visa",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<
        UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a>,
    >,
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
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    /// Configuration for eu_bank_transfer funding type.
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>>,
    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
#[serde(skip_serializing_if = "Option::is_none")]
pub type_: Option<&'a str>,

}
impl<'a> UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
    'a,
> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a>
    UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
        'a,
    >
{
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// This sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini {}
impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<
        UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a>,
    >,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<
        UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}
impl<'a> UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<&'a [UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions]>,

}
impl<'a>
    UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a>
{
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
Self::Balances => "balances",
Self::Ownership => "ownership",
Self::PaymentMethod => "payment_method",
Self::Transactions => "transactions",

        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of payment method types (e.g.
///
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    CustomerBalance,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    Paynow,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AchCreditTransfer => "ach_credit_transfer",
            Self::AchDebit => "ach_debit",
            Self::AcssDebit => "acss_debit",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Paynow => "paynow",
            Self::Promptpay => "promptpay",
            Self::SepaCreditTransfer => "sepa_credit_transfer",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Either `off`, or `on_subscription`.
///
/// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

impl UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::OnSubscription => "on_subscription",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies an interval for how often to bill for any pending invoice items.
///
/// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionPendingInvoiceItemInterval {
    /// Specifies invoicing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: UpdateSubscriptionPendingInvoiceItemIntervalInterval,
    /// The number of intervals between invoices.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpdateSubscriptionPendingInvoiceItemInterval {
    pub fn new(interval: UpdateSubscriptionPendingInvoiceItemIntervalInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies invoicing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPendingInvoiceItemIntervalInterval {
    Day,
    Month,
    Week,
    Year,
}

impl UpdateSubscriptionPendingInvoiceItemIntervalInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPendingInvoiceItemIntervalInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPendingInvoiceItemIntervalInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl UpdateSubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
///
/// This will be unset if you POST an empty value.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> UpdateSubscriptionTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount_percent: Default::default(), destination }
    }
}
/// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
///
/// This will always overwrite any trials that might apply via a subscribed plan.
/// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
/// The special value `now` can be provided to end the customer's trial immediately.
/// Can be at most two years from `billing_cycle_anchor`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum UpdateSubscriptionTrialEnd {
    Now,
    Timestamp(stripe_types::Timestamp),
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
