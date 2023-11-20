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
impl<'a> SearchSubscription<'a> {
    /// Search for subscriptions you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
    /// Don’t use search in read-after-write flows where strict consistency is necessary.
    ///
    /// Under normal operating conditions, data is searchable in less than a minute.
    /// Occasionally, propagation of new or updated data can be up to an hour behind during outages.
    /// Search functionality is not available to merchants in India.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<SearchReturned> {
        client.get_query("/subscriptions/search", self)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchReturned {
    pub data: Vec<stripe_types::Subscription>,
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
#[derive(Copy, Clone, Eq, PartialEq)]
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SearchReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SearchReturnedObject"))
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListSubscription<'a> {
    /// Filter subscriptions by their automatic tax settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<ListSubscriptionAutomaticTax>,
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
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
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
/// Filter subscriptions by their automatic tax settings.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListSubscriptionAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}
impl ListSubscriptionAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// The collection method of the subscriptions to retrieve.
///
/// Either `charge_automatically` or `send_invoice`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListSubscriptionCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl ListSubscriptionCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use ListSubscriptionCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for ListSubscriptionCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListSubscriptionCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListSubscriptionCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListSubscriptionCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The status of the subscriptions to retrieve.
///
/// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
/// Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://stripe.com/docs/billing/subscriptions/overview#subscription-statuses).
/// Passing in a value of `all` will return subscriptions of all statuses.
/// If no value is supplied, all subscriptions that have not been canceled are returned.
#[derive(Copy, Clone, Eq, PartialEq)]
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListSubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl<'a> ListSubscription<'a> {
    /// By default, returns a list of subscriptions that have not been canceled.
    ///
    /// In order to list canceled subscriptions, specify `status=canceled`.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::Subscription>> {
        client.get_query("/subscriptions", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::Subscription> {
        stripe::ListPaginator::from_params("/subscriptions", self)
    }
}
impl<'a> stripe::PaginationParams for ListSubscription<'a> {}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscription<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<&'a [CreateSubscriptionAddInvoiceItems<'a>]>,
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
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
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
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<CreateSubscriptionTrialSettings>,
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
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
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
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl CreateSubscriptionCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
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
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateSubscriptionItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionItemsPriceDataRecurringInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateSubscriptionItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl CreateSubscriptionPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentBehavior::*;
        match self {
            AllowIncomplete => "allow_incomplete",
            DefaultIncomplete => "default_incomplete",
            ErrorIfIncomplete => "error_if_incomplete",
            PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionPaymentBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentBehavior::*;
        match s {
            "allow_incomplete" => Ok(AllowIncomplete),
            "default_incomplete" => Ok(DefaultIncomplete),
            "error_if_incomplete" => Ok(ErrorIfIncomplete),
            "pending_if_incomplete" => Ok(PendingIfIncomplete),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionPaymentBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub konbini: Option<&'a serde_json::Value>,
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Fr => "fr",
            Nl => "nl",
        }
    }
}

impl std::str::FromStr
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Selected network to process this Subscription on.
///
/// Depends on the available networks of the card attached to the Subscription.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
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

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::*;
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

impl std::str::FromStr for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::*;
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

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
        }
    }
}

impl std::str::FromStr
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
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
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
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
    /// List of data features that you would like to retrieve upon account creation.
#[serde(skip_serializing_if = "Option::is_none")]
pub prefetch: Option<&'a [CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch]>,

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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match self {
Balances => "balances",
Ownership => "ownership",
PaymentMethod => "payment_method",
Transactions => "transactions",

        }
    }
}

impl std::str::FromStr for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match s {
    "balances" => Ok(Balances),
"ownership" => Ok(Ownership),
"payment_method" => Ok(PaymentMethod),
"transactions" => Ok(Transactions),
_ => Err(())

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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    Balances,
}

impl
    CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
    "balances" => Ok(Balances),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The list of payment method types (e.g.
///
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodTypes {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsPaymentMethodTypes::*;
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsPaymentMethodTypes::*;
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

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Either `off`, or `on_subscription`.
///
/// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

impl CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::*;
        match self {
            Off => "off",
            OnSubscription => "on_subscription",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::*;
        match s {
            "off" => Ok(Off),
            "on_subscription" => Ok(OnSubscription),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionPendingInvoiceItemIntervalInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateSubscriptionPendingInvoiceItemIntervalInterval {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionPendingInvoiceItemIntervalInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionPendingInvoiceItemIntervalInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionPendingInvoiceItemIntervalInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionPendingInvoiceItemIntervalInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionPendingInvoiceItemIntervalInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) resulting from the `billing_cycle_anchor`.
///
/// If no value is passed, the default is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl CreateSubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
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
/// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
/// The special value `now` can be provided to end the customer's trial immediately.
/// Can be at most two years from `billing_cycle_anchor`.
/// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum CreateSubscriptionTrialEnd {
    Now,
    StripeTypesTimestamp(stripe_types::Timestamp),
}
/// Settings related to subscription trials.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: CreateSubscriptionTrialSettingsEndBehavior,
}
impl CreateSubscriptionTrialSettings {
    pub fn new(end_behavior: CreateSubscriptionTrialSettingsEndBehavior) -> Self {
        Self { end_behavior }
    }
}
/// Defines how the subscription should behave when the user's free trial ends.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
}
impl CreateSubscriptionTrialSettingsEndBehavior {
    pub fn new(
        missing_payment_method: CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
    ) -> Self {
        Self { missing_payment_method }
    }
}
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateSubscription<'a> {
    /// Creates a new subscription on an existing customer.
    ///
    /// Each customer can have up to 500 active or scheduled subscriptions.  When you create a subscription with `collection_method=charge_automatically`, the first invoice is finalized as part of the request. The `payment_behavior` parameter determines the exact behavior of the initial payment.  To start subscriptions where the first invoice always begins in a `draft` status, use [subscription schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules#managing) instead. Schedules provide the flexibility to model more complex billing configurations that change over time.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::Subscription> {
        client.send_form("/subscriptions", self, http_types::Method::Post)
    }
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
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
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
    /// Details about why this subscription was cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_details: Option<UpdateSubscriptionCancellationDetails<'a>>,
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
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
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
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpdateSubscriptionProrationBehavior>,
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
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<UpdateSubscriptionTrialSettings>,
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
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
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
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionAddInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
}

impl UpdateSubscriptionBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionBillingCycleAnchor::*;
        match self {
            Now => "now",
            Unchanged => "unchanged",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionBillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionBillingCycleAnchor::*;
        match s {
            "now" => Ok(Now),
            "unchanged" => Ok(Unchanged),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
/// Details about why this subscription was cancelled.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionCancellationDetails<'a> {
    /// Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<&'a str>,
    /// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<UpdateSubscriptionCancellationDetailsFeedback>,
}
impl<'a> UpdateSubscriptionCancellationDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionCancellationDetailsFeedback {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl UpdateSubscriptionCancellationDetailsFeedback {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionCancellationDetailsFeedback::*;
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

impl std::str::FromStr for UpdateSubscriptionCancellationDetailsFeedback {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionCancellationDetailsFeedback::*;
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

impl AsRef<str> for UpdateSubscriptionCancellationDetailsFeedback {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionCancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionCancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionCancellationDetailsFeedback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl UpdateSubscriptionCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
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
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl UpdateSubscriptionItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionItemsPriceDataRecurringInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateSubscriptionItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionPauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl UpdateSubscriptionPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentBehavior::*;
        match self {
            AllowIncomplete => "allow_incomplete",
            DefaultIncomplete => "default_incomplete",
            ErrorIfIncomplete => "error_if_incomplete",
            PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionPaymentBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentBehavior::*;
        match s {
            "allow_incomplete" => Ok(AllowIncomplete),
            "default_incomplete" => Ok(DefaultIncomplete),
            "error_if_incomplete" => Ok(ErrorIfIncomplete),
            "pending_if_incomplete" => Ok(PendingIfIncomplete),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionPaymentBehavior {
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
    pub konbini: Option<&'a serde_json::Value>,
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Fr => "fr",
            Nl => "nl",
        }
    }
}

impl std::str::FromStr
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Selected network to process this Subscription on.
///
/// Depends on the available networks of the card attached to the Subscription.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
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

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::*;
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

impl std::str::FromStr for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::*;
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

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
        }
    }
}

impl std::str::FromStr
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
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
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
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
    /// List of data features that you would like to retrieve upon account creation.
#[serde(skip_serializing_if = "Option::is_none")]
pub prefetch: Option<&'a [UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch]>,

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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match self {
Balances => "balances",
Ownership => "ownership",
PaymentMethod => "payment_method",
Transactions => "transactions",

        }
    }
}

impl std::str::FromStr for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match s {
    "balances" => Ok(Balances),
"ownership" => Ok(Ownership),
"payment_method" => Ok(PaymentMethod),
"transactions" => Ok(Transactions),
_ => Err(())

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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    Balances,
}

impl
    UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
    "balances" => Ok(Balances),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The list of payment method types (e.g.
///
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsPaymentMethodTypes::*;
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsPaymentMethodTypes::*;
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

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Either `off`, or `on_subscription`.
///
/// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

impl UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::*;
        match self {
            Off => "off",
            OnSubscription => "on_subscription",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::*;
        match s {
            "off" => Ok(Off),
            "on_subscription" => Ok(OnSubscription),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionPendingInvoiceItemIntervalInterval {
    Day,
    Month,
    Week,
    Year,
}

impl UpdateSubscriptionPendingInvoiceItemIntervalInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionPendingInvoiceItemIntervalInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionPendingInvoiceItemIntervalInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionPendingInvoiceItemIntervalInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionPendingInvoiceItemIntervalInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionPendingInvoiceItemIntervalInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
///
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl UpdateSubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
///
/// This will be unset if you POST an empty value.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
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
#[serde(untagged)]
pub enum UpdateSubscriptionTrialEnd {
    Now,
    StripeTypesTimestamp(stripe_types::Timestamp),
}
/// Settings related to subscription trials.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: UpdateSubscriptionTrialSettingsEndBehavior,
}
impl UpdateSubscriptionTrialSettings {
    pub fn new(end_behavior: UpdateSubscriptionTrialSettingsEndBehavior) -> Self {
        Self { end_behavior }
    }
}
/// Defines how the subscription should behave when the user's free trial ends.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
}
impl UpdateSubscriptionTrialSettingsEndBehavior {
    pub fn new(
        missing_payment_method: UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
    ) -> Self {
        Self { missing_payment_method }
    }
}
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> UpdateSubscription<'a> {
    /// Updates an existing subscription to match the specified parameters.
    /// When changing prices or quantities, we optionally prorate the price we charge next month to make up for any price changes.
    /// To preview how the proration is calculated, use the [upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) endpoint.
    ///
    /// By default, we prorate subscription changes.
    ///
    /// For example, if a customer signs up on May 1 for a $100 price, they’ll be billed $100 immediately.
    /// If on May 15 they switch to a $200 price, then on June 1 they’ll be billed $250 ($200 for a renewal of her subscription, plus a $50 prorating adjustment for half of the previous month’s $100 difference).
    /// Similarly, a downgrade generates a credit that is applied to the next invoice.
    /// We also prorate when you make quantity changes.  Switching prices does not normally change the billing date or generate an immediate charge unless:  <ul> <li>The billing interval is changed (for example, from monthly to yearly).</li> <li>The subscription moves from free to paid, or paid to free.</li> <li>A trial starts or ends.</li> </ul>  In these cases, we apply a credit for the unused time on the previous price, immediately charge the customer using the new price, and reset the billing date.  If you want to charge for an upgrade immediately, pass `proration_behavior` as `always_invoice` to create prorations, automatically invoice the customer for those proration adjustments, and attempt to collect payment.
    /// If you pass `create_prorations`, the prorations are created but not automatically invoiced.
    /// If you want to bill the customer for the prorations before the subscription’s renewal date, you need to manually [invoice the customer](https://stripe.com/docs/api/invoices/create).  If you don’t want to prorate, set the `proration_behavior` option to `none`.
    /// With this option, the customer is billed $100 on May 1 and $200 on June 1.
    /// Similarly, if you set `proration_behavior` to `none` when switching between different billing intervals (for example, from monthly to yearly), we don’t generate any credits for the old subscription’s unused time.
    /// We still reset the billing date and bill immediately for the new subscription.  Updating the quantity on a subscription many times in an hour may result in [rate limiting](https://stripe.com/docs/rate-limits).
    /// If you need to bill for a frequently changing quantity, consider integrating [usage-based billing](https://stripe.com/docs/billing/subscriptions/usage-based) instead.
    pub fn send(
        &self,
        client: &stripe::Client,
        subscription_exposed_id: &stripe_types::subscription::SubscriptionId,
    ) -> stripe::Response<stripe_types::Subscription> {
        client.send_form(
            &format!(
                "/subscriptions/{subscription_exposed_id}",
                subscription_exposed_id = subscription_exposed_id
            ),
            self,
            http_types::Method::Post,
        )
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
impl<'a> RetrieveSubscription<'a> {
    /// Retrieves the subscription with the given ID.
    pub fn send(
        &self,
        client: &stripe::Client,
        subscription_exposed_id: &stripe_types::subscription::SubscriptionId,
    ) -> stripe::Response<stripe_types::Subscription> {
        client.get_query(
            &format!(
                "/subscriptions/{subscription_exposed_id}",
                subscription_exposed_id = subscription_exposed_id
            ),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelSubscription<'a> {
    /// Details about why this subscription was cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_details: Option<CancelSubscriptionCancellationDetails<'a>>,
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
/// Details about why this subscription was cancelled.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelSubscriptionCancellationDetails<'a> {
    /// Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<&'a str>,
    /// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<CancelSubscriptionCancellationDetailsFeedback>,
}
impl<'a> CancelSubscriptionCancellationDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CancelSubscriptionCancellationDetailsFeedback {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl CancelSubscriptionCancellationDetailsFeedback {
    pub fn as_str(self) -> &'static str {
        use CancelSubscriptionCancellationDetailsFeedback::*;
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

impl std::str::FromStr for CancelSubscriptionCancellationDetailsFeedback {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CancelSubscriptionCancellationDetailsFeedback::*;
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

impl AsRef<str> for CancelSubscriptionCancellationDetailsFeedback {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CancelSubscriptionCancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CancelSubscriptionCancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CancelSubscriptionCancellationDetailsFeedback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CancelSubscription<'a> {
    /// Cancels a customer’s subscription immediately.
    ///
    /// The customer will not be charged again for the subscription.  Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually [deleted](https://stripe.com/docs/api#delete_invoiceitem).
    /// If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period.
    /// But if the subscription is set to cancel immediately, pending prorations will be removed.  By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer.
    /// This is intended to prevent unexpected payment attempts after the customer has canceled a subscription.
    /// However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed.
    /// Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.
    pub fn send(
        &self,
        client: &stripe::Client,
        subscription_exposed_id: &stripe_types::subscription::SubscriptionId,
    ) -> stripe::Response<stripe_types::Subscription> {
        client.send_form(
            &format!(
                "/subscriptions/{subscription_exposed_id}",
                subscription_exposed_id = subscription_exposed_id
            ),
            self,
            http_types::Method::Delete,
        )
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
    pub billing_cycle_anchor: Option<ResumeSubscriptionBillingCycleAnchor>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<ResumeSubscriptionProrationBehavior>,
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
/// Either `now` or `unchanged`.
///
/// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
/// Setting the value to `unchanged` advances the subscription's billing cycle anchor to the period that surrounds the current time.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ResumeSubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
}

impl ResumeSubscriptionBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use ResumeSubscriptionBillingCycleAnchor::*;
        match self {
            Now => "now",
            Unchanged => "unchanged",
        }
    }
}

impl std::str::FromStr for ResumeSubscriptionBillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ResumeSubscriptionBillingCycleAnchor::*;
        match s {
            "now" => Ok(Now),
            "unchanged" => Ok(Unchanged),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ResumeSubscriptionBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ResumeSubscriptionBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ResumeSubscriptionBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ResumeSubscriptionBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
///
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ResumeSubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl ResumeSubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use ResumeSubscriptionProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for ResumeSubscriptionProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ResumeSubscriptionProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ResumeSubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ResumeSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ResumeSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ResumeSubscriptionProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ResumeSubscription<'a> {
    /// Initiates resumption of a paused subscription, optionally resetting the billing cycle anchor and creating prorations.
    ///
    /// If a resumption invoice is generated, it must be paid or marked uncollectible before the subscription will be unpaused.
    /// If payment succeeds the subscription will become `active`, and if payment fails the subscription will be `past_due`.
    /// The resumption invoice will void automatically if not paid by the expiration date.
    pub fn send(
        &self,
        client: &stripe::Client,
        subscription: &stripe_types::subscription::SubscriptionId,
    ) -> stripe::Response<stripe_types::Subscription> {
        client.send_form(
            &format!("/subscriptions/{subscription}/resume", subscription = subscription),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteDiscountSubscription {}
impl DeleteDiscountSubscription {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteDiscountSubscription {
    /// Removes the currently applied discount on a subscription.
    pub fn send(
        &self,
        client: &stripe::Client,
        subscription_exposed_id: &stripe_types::subscription::SubscriptionId,
    ) -> stripe::Response<stripe_types::DeletedDiscount> {
        client.send_form(
            &format!(
                "/subscriptions/{subscription_exposed_id}/discount",
                subscription_exposed_id = subscription_exposed_id
            ),
            self,
            http_types::Method::Delete,
        )
    }
}
