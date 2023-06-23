use stripe::{Client, Response};

impl stripe_core::subscription_item::SubscriptionItem {
    /// Returns a list of your subscription items for a given subscription.
    pub fn list(
        client: &Client,
        params: ListSubscriptionItem,
    ) -> Response<stripe_types::List<stripe_core::subscription_item::SubscriptionItem>> {
        client.get_query("/subscription_items", params)
    }
    /// Retrieves the subscription item with the given ID.
    pub fn retrieve(
        client: &Client,
        item: &stripe_core::line_item::ItemId,
        params: RetrieveSubscriptionItem,
    ) -> Response<stripe_core::subscription_item::SubscriptionItem> {
        client.get_query(&format!("/subscription_items/{item}", item = item), params)
    }
    /// Adds a new item to an existing subscription.
    ///
    /// No existing items will be changed or replaced.
    pub fn create(
        client: &Client,
        params: CreateSubscriptionItem,
    ) -> Response<stripe_core::subscription_item::SubscriptionItem> {
        client.send_form("/subscription_items", params, http_types::Method::Post)
    }
    /// Updates the plan or quantity of an item on a current subscription.
    pub fn update(
        client: &Client,
        item: &stripe_core::line_item::ItemId,
        params: UpdateSubscriptionItem,
    ) -> Response<stripe_core::subscription_item::SubscriptionItem> {
        client.send_form(
            &format!("/subscription_items/{item}", item = item),
            params,
            http_types::Method::Post,
        )
    }
    /// Deletes an item from the subscription.
    ///
    /// Removing a subscription item from a subscription will not cancel the subscription.
    pub fn delete(
        client: &Client,
        item: &stripe_core::line_item::ItemId,
        params: DeleteSubscriptionItem,
    ) -> Response<stripe_core::subscription_item::DeletedSubscriptionItem> {
        client.send_form(
            &format!("/subscription_items/{item}", item = item),
            params,
            http_types::Method::Delete,
        )
    }
    /// For the specified subscription item, returns a list of summary objects.
    ///
    /// Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).  The list is sorted in reverse-chronological order (newest first).
    /// The first list item represents the most current usage period that hasn’t ended yet.
    /// Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.
    pub fn usage_record_summaries(
        client: &Client,
        subscription_item: &stripe_core::subscription_item::SubscriptionItemId,
        params: UsageRecordSummariesSubscriptionItem,
    ) -> Response<stripe_types::List<stripe_core::usage_record_summary::UsageRecordSummary>> {
        client.get_query(
            &format!(
                "/subscription_items/{subscription_item}/usage_record_summaries",
                subscription_item = subscription_item
            ),
            params,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListSubscriptionItem<'a> {
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
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// The ID of the subscription whose items will be retrieved.
    pub subscription: &'a str,
}
impl<'a> ListSubscriptionItem<'a> {
    pub fn new(subscription: &'a str) -> Self {
        Self {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            subscription,
        }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveSubscriptionItem<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSubscriptionItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionItem<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<CreateSubscriptionItemBillingThresholds>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
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
    pub payment_behavior: Option<CreateSubscriptionItemPaymentBehavior>,
    /// The identifier of the plan to add to the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateSubscriptionItemPriceData<'a>>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateSubscriptionItemProrationBehavior>,
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<stripe_types::Timestamp>,
    /// The quantity you'd like to apply to the subscription item you're creating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The identifier of the subscription to modify.
    pub subscription: &'a str,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateSubscriptionItem<'a> {
    pub fn new(subscription: &'a str) -> Self {
        Self {
            billing_thresholds: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            payment_behavior: Default::default(),
            plan: Default::default(),
            price: Default::default(),
            price_data: Default::default(),
            proration_behavior: Default::default(),
            proration_date: Default::default(),
            quantity: Default::default(),
            subscription,
            tax_rates: Default::default(),
        }
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
///
/// When updating, pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionItemBillingThresholds {
    /// Usage threshold that triggers the subscription to advance to a new billing period.
    pub usage_gte: i64,
}
impl CreateSubscriptionItemBillingThresholds {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
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
pub enum CreateSubscriptionItemPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl CreateSubscriptionItemPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllowIncomplete => "allow_incomplete",
            Self::DefaultIncomplete => "default_incomplete",
            Self::ErrorIfIncomplete => "error_if_incomplete",
            Self::PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl AsRef<str> for CreateSubscriptionItemPaymentBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionItemPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionItemPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: CreateSubscriptionItemPriceDataRecurring,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSubscriptionItemPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateSubscriptionItemPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: CreateSubscriptionItemPriceDataRecurring,
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
pub struct CreateSubscriptionItemPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: CreateSubscriptionItemPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateSubscriptionItemPriceDataRecurring {
    pub fn new(interval: CreateSubscriptionItemPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionItemPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateSubscriptionItemPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for CreateSubscriptionItemPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionItemPriceDataRecurringInterval {
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
pub enum CreateSubscriptionItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateSubscriptionItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateSubscriptionItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionItemProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl CreateSubscriptionItemProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSubscriptionItemProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionItemProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionItem<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpdateSubscriptionItemBillingThresholds>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
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
    pub payment_behavior: Option<UpdateSubscriptionItemPaymentBehavior>,
    /// The identifier of the new plan for this subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// The ID of the price object.
    ///
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateSubscriptionItemPriceData<'a>>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpdateSubscriptionItemProrationBehavior>,
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<stripe_types::Timestamp>,
    /// The quantity you'd like to apply to the subscription item you're creating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpdateSubscriptionItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
///
/// When updating, pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionItemBillingThresholds {
    /// Usage threshold that triggers the subscription to advance to a new billing period.
    pub usage_gte: i64,
}
impl UpdateSubscriptionItemBillingThresholds {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
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
pub enum UpdateSubscriptionItemPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl UpdateSubscriptionItemPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllowIncomplete => "allow_incomplete",
            Self::DefaultIncomplete => "default_incomplete",
            Self::ErrorIfIncomplete => "error_if_incomplete",
            Self::PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionItemPaymentBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionItemPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionItemPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpdateSubscriptionItemPriceDataRecurring,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateSubscriptionItemPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateSubscriptionItemPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: UpdateSubscriptionItemPriceDataRecurring,
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
pub struct UpdateSubscriptionItemPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: UpdateSubscriptionItemPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpdateSubscriptionItemPriceDataRecurring {
    pub fn new(interval: UpdateSubscriptionItemPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionItemPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl UpdateSubscriptionItemPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionItemPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionItemPriceDataRecurringInterval {
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
pub enum UpdateSubscriptionItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateSubscriptionItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionItemProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl UpdateSubscriptionItemProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionItemProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionItemProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteSubscriptionItem {
    /// Delete all usage for the given subscription item.
    ///
    /// Allowed only when the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<DeleteSubscriptionItemProrationBehavior>,
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<stripe_types::Timestamp>,
}
impl DeleteSubscriptionItem {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteSubscriptionItemProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl DeleteSubscriptionItemProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for DeleteSubscriptionItemProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeleteSubscriptionItemProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UsageRecordSummariesSubscriptionItem<'a> {
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
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> UsageRecordSummariesSubscriptionItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
