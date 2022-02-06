// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CouponId, CustomerId, PriceId, PromotionCodeId};
use crate::params::{Deleted, Expand, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    CollectionMethod, Currency, Scheduled, SubscriptionBillingThresholds,
    SubscriptionItemBillingThresholds,
};
use crate::{InvoiceItemPriceData, Subscription};

impl Subscription {
    /// By default, returns a list of subscriptions that have not been canceled.
    ///
    /// In order to list canceled subscriptions, specify `status=canceled`.
    pub fn list(client: &Client, params: ListSubscriptions<'_>) -> Response<List<Subscription>> {
        client.get_query("/subscriptions", &params)
    }

    /// Creates a new subscription on an existing customer.
    ///
    /// Each customer can have up to 500 active or scheduled subscriptions.
    pub fn create(client: &Client, params: CreateSubscription<'_>) -> Response<Subscription> {
        client.post_form("/subscriptions", &params)
    }

    /// Retrieves the subscription with the given ID.
    pub fn retrieve(
        client: &Client,
        id: &SubscriptionId,
        expand: &[&str],
    ) -> Response<Subscription> {
        client.get_query(&format!("/subscriptions/{}", id), &Expand { expand })
    }

    /// Updates an existing subscription on a customer to match the specified parameters.
    ///
    /// When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes.
    /// To preview how the proration will be calculated, use the [upcoming invoice](https://stripe.com/docs/api#upcoming_invoice) endpoint.
    pub fn update(
        client: &Client,
        id: &SubscriptionId,
        params: UpdateSubscription<'_>,
    ) -> Response<Subscription> {
        client.post_form(&format!("/subscriptions/{}", id), &params)
    }

    /// Cancels a customer’s subscription immediately.
    ///
    /// The customer will not be charged again for the subscription.  Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually [deleted](https://stripe.com/docs/api#delete_invoiceitem).
    /// If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period.
    /// But if the subscription is set to cancel immediately, pending prorations will be removed.  By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer.
    /// This is intended to prevent unexpected payment attempts after the customer has canceled a subscription.
    /// However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed.
    /// Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.
    pub fn delete(client: &Client, id: &SubscriptionId) -> Response<Deleted<SubscriptionId>> {
        client.delete(&format!("/subscriptions/{}", id))
    }
}

impl Object for Subscription {
    type Id = SubscriptionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription"
    }
}

// written at 597
/// The parameters for `Subscription::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateSubscription<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the first invoice for this subscription.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Box<Vec<AddInvoiceItems>>>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Automatic tax settings for this subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<Box<CreateSubscriptionAutomaticTax>>,

    /// For new subscriptions, a past timestamp to backdate the subscription's start date to.
    ///
    /// If set, the first invoice will contain a proration for the timespan between the start date and the current time.
    /// Can be combined with trials and the billing cycle anchor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backdate_start_date: Option<Timestamp>,

    /// A future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    ///
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<Timestamp>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// A timestamp at which the subscription should cancel.
    ///
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<Timestamp>,

    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// The ID of the coupon to apply to this subscription.
    ///
    /// A coupon applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// The identifier of the customer to subscribe.
    pub customer: CustomerId,

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
    pub default_tax_rates: Option<Box<Vec<String>>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Vec<CreateSubscriptionItems>>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<bool>,

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
    /// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.  `pending_if_incomplete` is only used with updates and cannot be passed when creating a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_behavior: Option<SubscriptionPaymentBehavior>,

    /// Payment settings to pass to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<Box<CreateSubscriptionPaymentSettings>>,

    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<Box<CreateSubscriptionPendingInvoiceItemInterval>>,

    /// The API ID of a promotion code to apply to this subscription.
    ///
    /// A promotion code applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<PromotionCodeId>,

    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) resulting from the `billing_cycle_anchor`.
    ///
    /// Valid values are `create_prorations` or `none`.  Passing `create_prorations` will cause proration invoice items to be created when applicable.
    /// Prorations can be disabled by passing `none`.
    /// If no value is passed, the default is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<SubscriptionProrationBehavior>,

    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<CreateSubscriptionTransferData>>,

    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,

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
    pub fn new(customer: CustomerId) -> Self {
        CreateSubscription {
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
            customer,
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            expand: Default::default(),
            items: Default::default(),
            metadata: Default::default(),
            off_session: Default::default(),
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

// written at 597
/// The parameters for `Subscription::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListSubscriptions<'a> {
    /// The collection method of the subscriptions to retrieve.
    ///
    /// Either `charge_automatically` or `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<RangeQuery<Timestamp>>,

    /// The ID of the customer whose subscriptions will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SubscriptionId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Filter for subscriptions that contain this recurring price ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SubscriptionId>,

    /// The status of the subscriptions to retrieve.
    ///
    /// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
    /// Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://stripe.com/docs/billing/subscriptions/overview#subscription-statuses).
    /// Passing in a value of `all` will return subscriptions of all statuses.
    /// If no value is supplied, all subscriptions that have not been canceled are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatus>,
}

impl<'a> ListSubscriptions<'a> {
    pub fn new() -> Self {
        ListSubscriptions {
            collection_method: Default::default(),
            created: Default::default(),
            current_period_end: Default::default(),
            current_period_start: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            price: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}

// written at 597
/// The parameters for `Subscription::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSubscription<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the first invoice for this subscription.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Box<Vec<AddInvoiceItems>>>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Automatic tax settings for this subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<Box<UpdateSubscriptionAutomaticTax>>,

    /// Either `now` or `unchanged`.
    ///
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<SubscriptionBillingCycleAnchor>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// A timestamp at which the subscription should cancel.
    ///
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<Box<Timestamp>>,

    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// The ID of the coupon to apply to this subscription.
    ///
    /// A coupon applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

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
    pub default_tax_rates: Option<Box<Vec<String>>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Vec<UpdateSubscriptionItems>>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<bool>,

    /// If specified, payment collection for this subscription will be paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_collection: Option<Box<UpdateSubscriptionPauseCollection>>,

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
    pub payment_behavior: Option<SubscriptionPaymentBehavior>,

    /// Payment settings to pass to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<Box<UpdateSubscriptionPaymentSettings>>,

    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<Box<UpdateSubscriptionPendingInvoiceItemInterval>>,

    /// The promotion code to apply to this subscription.
    ///
    /// A promotion code applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<PromotionCodeId>,

    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    ///
    /// Valid values are `create_prorations`, `none`, or `always_invoice`.  Passing `create_prorations` will cause proration invoice items to be created when applicable.
    /// These proration items will only be invoiced immediately under [certain conditions](https://stripe.com/docs/subscriptions/upgrading-downgrading#immediate-payment).
    /// In order to always invoice immediately for prorations, pass `always_invoice`.  Prorations can be disabled by passing `none`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<SubscriptionProrationBehavior>,

    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply exactly the same proration that was previewed with [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    /// It can also be used to implement custom proration logic, such as prorating by day instead of by second, by providing the time that you wish to use for proration calculations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<Timestamp>,

    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    ///
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<UpdateSubscriptionTransferData>>,

    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,

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
        UpdateSubscription {
            add_invoice_items: Default::default(),
            application_fee_percent: Default::default(),
            automatic_tax: Default::default(),
            billing_cycle_anchor: Default::default(),
            billing_thresholds: Default::default(),
            cancel_at: Default::default(),
            cancel_at_period_end: Default::default(),
            collection_method: Default::default(),
            coupon: Default::default(),
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            expand: Default::default(),
            items: Default::default(),
            metadata: Default::default(),
            off_session: Default::default(),
            pause_collection: Default::default(),
            payment_behavior: Default::default(),
            payment_settings: Default::default(),
            pending_invoice_item_interval: Default::default(),
            promotion_code: Default::default(),
            proration_behavior: Default::default(),
            proration_date: Default::default(),
            transfer_data: Default::default(),
            trial_end: Default::default(),
            trial_from_plan: Default::default(),
        }
    }
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddInvoiceItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<Box<InvoiceItemPriceData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionAutomaticTax {
    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<Box<CreateSubscriptionItemsBillingThresholds>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<Box<SubscriptionItemPriceData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<Box<CreateSubscriptionPaymentSettingsPaymentMethodOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Box<Vec<CreateSubscriptionPaymentSettingsPaymentMethodTypes>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionPendingInvoiceItemInterval {
    pub interval: PlanInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<Box<u64>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<Box<f64>>,

    pub destination: String,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionAutomaticTax {
    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<Box<SubscriptionItemPriceData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionPauseCollection {
    pub behavior: UpdateSubscriptionPauseCollectionBehavior,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumes_at: Option<Box<Timestamp>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Box<Vec<UpdateSubscriptionPaymentSettingsPaymentMethodTypes>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionPendingInvoiceItemInterval {
    pub interval: PlanInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<Box<u64>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<Box<f64>>,

    pub destination: String,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionItemsBillingThresholds {
    pub usage_gte: i64,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItemPriceData {
    pub currency: Currency,

    pub product: String,

    pub recurring: SubscriptionItemPriceDataRecurring,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<Box<SubscriptionItemPriceDataTaxBehavior>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<
        Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
    >,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<
        Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
    >,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItemPriceDataRecurring {
    pub interval: PlanInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<Box<u64>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<
        Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
    >,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<
        Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
    >,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<
        Box<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType>,
    >,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<
        Box<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType>,
    >,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
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

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
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

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
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

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions`'s `amount_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Fixed => "fixed",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Maximum => "maximum",
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

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
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

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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
    Fpx,
    Giropay,
    Ideal,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::AchCreditTransfer => {
                "ach_credit_transfer"
            }
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::AchDebit => "ach_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Boleto => "boleto",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Card => "card",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
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

/// An enum representing the possible values of an `CreateSubscriptionPendingInvoiceItemInterval`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanInterval {
    Day,
    Month,
    Week,
    Year,
}

impl PlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            PlanInterval::Day => "day",
            PlanInterval::Month => "month",
            PlanInterval::Week => "week",
            PlanInterval::Year => "year",
        }
    }
}

impl AsRef<str> for PlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `UpdateSubscription`'s `billing_cycle_anchor` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
}

impl SubscriptionBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionBillingCycleAnchor::Now => "now",
            SubscriptionBillingCycleAnchor::Unchanged => "unchanged",
        }
    }
}

impl AsRef<str> for SubscriptionBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SubscriptionItemPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl SubscriptionItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionItemPriceDataTaxBehavior::Exclusive => "exclusive",
            SubscriptionItemPriceDataTaxBehavior::Inclusive => "inclusive",
            SubscriptionItemPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for SubscriptionItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateSubscription`'s `payment_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl SubscriptionPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionPaymentBehavior::AllowIncomplete => "allow_incomplete",
            SubscriptionPaymentBehavior::DefaultIncomplete => "default_incomplete",
            SubscriptionPaymentBehavior::ErrorIfIncomplete => "error_if_incomplete",
            SubscriptionPaymentBehavior::PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl AsRef<str> for SubscriptionPaymentBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateSubscription`'s `proration_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl SubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionProrationBehavior::AlwaysInvoice => "always_invoice",
            SubscriptionProrationBehavior::CreateProrations => "create_prorations",
            SubscriptionProrationBehavior::None => "none",
        }
    }
}

impl AsRef<str> for SubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `ListSubscriptions`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
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

impl SubscriptionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::All => "all",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::Ended => "ended",
            SubscriptionStatus::Incomplete => "incomplete",
            SubscriptionStatus::IncompleteExpired => "incomplete_expired",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Trialing => "trialing",
            SubscriptionStatus::Unpaid => "unpaid",
        }
    }
}

impl AsRef<str> for SubscriptionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPauseCollection`'s `behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}

impl UpdateSubscriptionPauseCollectionBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPauseCollectionBehavior::KeepAsDraft => "keep_as_draft",
            UpdateSubscriptionPauseCollectionBehavior::MarkUncollectible => "mark_uncollectible",
            UpdateSubscriptionPauseCollectionBehavior::Void => "void",
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

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
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

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
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

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
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

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions`'s `amount_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Fixed => "fixed",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Maximum => "maximum",
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

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
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

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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
    Fpx,
    Giropay,
    Ideal,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::AchCreditTransfer => {
                "ach_credit_transfer"
            }
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::AchDebit => "ach_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Boleto => "boleto",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Card => "card",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
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
