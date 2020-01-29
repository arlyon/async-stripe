// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CustomerId, SubscriptionScheduleId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    CollectionMethod, Coupon, Customer, PaymentMethod, Plan, PlanInterval, Scheduled, Subscription,
    SubscriptionBillingThresholds, SubscriptionItemBillingThresholds, TaxRate,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionSchedule".
///
/// For more details see [https://stripe.com/docs/api/subscription_schedules/object](https://stripe.com/docs/api/subscription_schedules/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedule {
    /// Unique identifier for the object.
    pub id: SubscriptionScheduleId,

    /// This field has been renamed to `collection_method` and will be removed in a future API version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<CollectionMethod>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// Time at which the subscription schedule was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// Time at which the subscription schedule was completed.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<Timestamp>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<SubscriptionScheduleCurrentPhase>,

    /// ID of the customer who owns the subscription schedule.
    pub customer: Expandable<Customer>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    /// Behavior of the subscription schedule and underlying subscription when it ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<SubscriptionScheduleEndBehavior>,

    /// The subscription schedule's default invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Configuration for the subscription schedule's phases.
    pub phases: Vec<SubscriptionSchedulePhaseConfiguration>,

    /// Time at which the subscription schedule was released.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<Timestamp>,

    /// ID of the subscription once managed by the subscription schedule (if it is released).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_subscription: Option<String>,

    /// Behavior of the subscription schedule and underlying subscription when it ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_behavior: Option<SubscriptionScheduleRenewalBehavior>,

    /// Interval and duration at which the subscription schedule renews for when it ends if `renewal_behavior` is `renew`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_interval: Option<SubscriptionScheduleRenewalInterval>,

    /// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
    pub status: SubscriptionScheduleStatus,

    /// ID of the subscription managed by the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,
}

impl SubscriptionSchedule {
    /// Retrieves the list of your subscription schedules.
    pub fn list(
        client: &Client,
        params: ListSubscriptionSchedules<'_>,
    ) -> Response<List<SubscriptionSchedule>> {
        client.get_query("/subscription_schedules", &params)
    }

    /// Creates a new subscription schedule object.
    pub fn create(
        client: &Client,
        params: CreateSubscriptionSchedule<'_>,
    ) -> Response<SubscriptionSchedule> {
        client.post_form("/subscription_schedules", &params)
    }

    /// Retrieves the details of an existing subscription schedule.
    ///
    /// You only need to supply the unique subscription schedule identifier that was returned upon subscription schedule creation.
    pub fn retrieve(
        client: &Client,
        id: &SubscriptionScheduleId,
        expand: &[&str],
    ) -> Response<SubscriptionSchedule> {
        client.get_query(&format!("/subscription_schedules/{}", id), &Expand { expand })
    }

    /// Updates an existing subscription schedule.
    pub fn update(
        client: &Client,
        id: &SubscriptionScheduleId,
        params: UpdateSubscriptionSchedule<'_>,
    ) -> Response<SubscriptionSchedule> {
        client.post_form(&format!("/subscription_schedules/{}", id), &params)
    }
}

impl Object for SubscriptionSchedule {
    type Id = SubscriptionScheduleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_schedule"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleCurrentPhase {
    pub end_date: Timestamp,

    pub start_date: Timestamp,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedulePhaseConfiguration {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account during this phase of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// ID of the coupon to use during this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Expandable<Coupon>>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,

    /// The end of this phase of the subscription schedule.
    pub end_date: Timestamp,

    /// The subscription schedule's default invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    /// Plans to subscribe during this phase of the subscription schedule.
    pub plans: Vec<SubscriptionScheduleConfigurationItem>,

    /// The start of this phase of the subscription schedule.
    pub start_date: Timestamp,

    /// If provided, each invoice created during this phase of the subscription schedule will apply the tax rate, increasing the amount billed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    /// When the trial ends within the phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleConfigurationItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    /// ID of the plan to which the customer should be subscribed.
    pub plan: Expandable<Plan>,

    /// Quantity of the plan to which the customer should be subscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to this `phase_item`.
    ///
    /// When set, the `default_tax_rates` on the phase do not apply to this `phase_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleRenewalInterval {
    /// Interval at which to renew the subscription schedule for when it ends.
    pub interval: PlanInterval,

    /// Number of intervals to renew the subscription schedule for when it ends.
    pub length: i64,
}

/// The parameters for `SubscriptionSchedule::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateSubscriptionSchedule<'a> {
    /// This field has been renamed to `collection_method` and will be removed in a future API version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<CollectionMethod>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// The identifier of the customer to create the subscription schedule for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,

    /// ID of the default payment source for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule and be in a chargeable state.
    /// If not set, defaults to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<SubscriptionScheduleRenewalBehavior>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Migrate an existing subscription to be managed by a subscription schedule.
    ///
    /// If this parameter is set, a subscription schedule will be created using the subscription's plan(s), set to auto-renew using the subscription's interval.
    /// Other parameters cannot be set since their values will be inferred from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_subscription: Option<&'a str>,

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// List representing phases of the subscription schedule.
    ///
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<CreateSubscriptionSchedulePhases>>,

    /// Configures how the subscription schedule behaves when it ends.
    ///
    /// Possible values are `none`, `cancel`, `renew`, or `release`.
    /// `renew` will create a new subscription schedule revision by adding a new phase using the most recent phase's `plans` applied to a duration set by `renewal_interval`.
    /// `none` will stop the subscription schedule and cancel the underlying subscription.
    /// `cancel` is semantically the same as `none`.
    /// `release` will stop the subscription schedule, but keep the underlying subscription running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_behavior: Option<SubscriptionScheduleRenewalBehavior>,

    /// Configuration for renewing the subscription schedule when it ends.
    ///
    /// Must be set if `renewal_behavior` is `renew`.
    /// Otherwise, must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_interval: Option<SubscriptionScheduleRenewalIntervalParams>,

    /// The date at which the subscription schedule starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Scheduled>,
}

impl<'a> CreateSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        CreateSubscriptionSchedule {
            billing: Default::default(),
            billing_thresholds: Default::default(),
            collection_method: Default::default(),
            customer: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            end_behavior: Default::default(),
            expand: Default::default(),
            from_subscription: Default::default(),
            invoice_settings: Default::default(),
            metadata: Default::default(),
            phases: Default::default(),
            renewal_behavior: Default::default(),
            renewal_interval: Default::default(),
            start_date: Default::default(),
        }
    }
}

/// The parameters for `SubscriptionSchedule::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListSubscriptionSchedules<'a> {
    /// Only return subscription schedules that were created canceled the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<RangeQuery<Timestamp>>,

    /// Only return subscription schedules that completed during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<RangeQuery<Timestamp>>,

    /// Only return subscription schedules that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return subscription schedules for the given customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SubscriptionScheduleId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return subscription schedules that were released during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<RangeQuery<Timestamp>>,

    /// Only return subscription schedules that have not started yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SubscriptionScheduleId>,
}

impl<'a> ListSubscriptionSchedules<'a> {
    pub fn new() -> Self {
        ListSubscriptionSchedules {
            canceled_at: Default::default(),
            completed_at: Default::default(),
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            released_at: Default::default(),
            scheduled: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `SubscriptionSchedule::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSubscriptionSchedule<'a> {
    /// This field has been renamed to `collection_method` and will be removed in a future API version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<CollectionMethod>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,

    /// ID of the default payment source for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule and be in a chargeable state.
    /// If not set, defaults to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<SubscriptionScheduleRenewalBehavior>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// List representing phases of the subscription schedule.
    ///
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    /// Note that past phases can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<UpdateSubscriptionSchedulePhases>>,

    /// If the update changes the current phase, indicates if the changes should be prorated.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,

    /// Configures how the subscription schedule behaves when it ends.
    ///
    /// Possible values are `none`, `cancel`, `renew`, or `release`.
    /// `renew` will create a new subscription schedule revision by adding a new phase using the most recent phase's `plans` applied to a duration set by `renewal_interval`.
    /// `none` will stop the subscription schedule and cancel the underlying subscription.
    /// `cancel` is semantically the same as `none`.
    /// `release` will stop the subscription schedule, but keep the underlying subscription running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_behavior: Option<SubscriptionScheduleRenewalBehavior>,

    /// Configuration for renewing the subscription schedule when it ends.
    ///
    /// Must be set if `renewal_behavior` is `renew`.
    /// Otherwise, must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_interval: Option<SubscriptionScheduleRenewalIntervalParams>,
}

impl<'a> UpdateSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        UpdateSubscriptionSchedule {
            billing: Default::default(),
            billing_thresholds: Default::default(),
            collection_method: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            end_behavior: Default::default(),
            expand: Default::default(),
            invoice_settings: Default::default(),
            metadata: Default::default(),
            phases: Default::default(),
            prorate: Default::default(),
            renewal_behavior: Default::default(),
            renewal_interval: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,

    pub plans: Vec<SubscriptionSchedulePhasesPlansParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleInvoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleRenewalIntervalParams {
    pub interval: PlanInterval,

    pub length: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,

    pub plans: Vec<SubscriptionSchedulePhasesPlansParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedulePhasesPlansParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    pub plan: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

/// An enum representing the possible values of an `SubscriptionSchedule`'s `end_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleEndBehavior {
    Cancel,
    Release,
}

impl SubscriptionScheduleEndBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionScheduleEndBehavior::Cancel => "cancel",
            SubscriptionScheduleEndBehavior::Release => "release",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleEndBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SubscriptionSchedule`'s `renewal_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleRenewalBehavior {
    Cancel,
    None,
    Release,
    Renew,
}

impl SubscriptionScheduleRenewalBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionScheduleRenewalBehavior::Cancel => "cancel",
            SubscriptionScheduleRenewalBehavior::None => "none",
            SubscriptionScheduleRenewalBehavior::Release => "release",
            SubscriptionScheduleRenewalBehavior::Renew => "renew",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleRenewalBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleRenewalBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SubscriptionSchedule`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleStatus {
    Active,
    Canceled,
    Completed,
    NotStarted,
    Released,
}

impl SubscriptionScheduleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionScheduleStatus::Active => "active",
            SubscriptionScheduleStatus::Canceled => "canceled",
            SubscriptionScheduleStatus::Completed => "completed",
            SubscriptionScheduleStatus::NotStarted => "not_started",
            SubscriptionScheduleStatus::Released => "released",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
