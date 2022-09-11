// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_common::resources::Currency;
use async_stripe_core::{
    ids::{CustomerId, SubscriptionScheduleId},
    params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp},
    resources::{CollectionMethod, Scheduled, SubscriptionBillingThresholds},
    BaseClient, Client,
};
use serde::{Deserialize, Serialize};

use crate::resources::{
    Application, Customer, Scheduled, Subscription, SubscriptionScheduleDefaultSettings,
    SubscriptionSchedulePhaseConfiguration, TestHelpersTestClock,
};

/// The resource representing a Stripe "SubscriptionSchedule".
///
/// For more details see <https://stripe.com/docs/api/subscription_schedules/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionSchedule {
    /// Unique identifier for the object.
    pub id: SubscriptionScheduleId,

    /// ID of the Connect Application that created the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Expandable<Application>>,

    /// Time at which the subscription schedule was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

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

    pub default_settings: SubscriptionScheduleDefaultSettings,

    /// Behavior of the subscription schedule and underlying subscription when it ends.
    ///
    /// Possible values are `release` and `cancel`.
    pub end_behavior: SubscriptionScheduleEndBehavior,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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

    /// The present status of the subscription schedule.
    ///
    /// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
    /// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
    pub status: SubscriptionScheduleStatus,

    /// ID of the subscription managed by the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,

    /// ID of the test clock this subscription schedule belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<Expandable<TestHelpersTestClock>>,
}

impl SubscriptionSchedule {
    /// Retrieves the list of your subscription schedules.
    pub fn list<C: BaseClient>(
        client: &Client<C>,
        params: &ListSubscriptionSchedules<'_>,
    ) -> <C as BaseClient>::Response<List<SubscriptionSchedule>> {
        client.get_query("/subscription_schedules", &params)
    }

    /// Creates a new subscription schedule object.
    ///
    /// Each customer can have up to 500 active or scheduled subscriptions.
    pub fn create<C: BaseClient>(
        client: &Client<C>,
        params: CreateSubscriptionSchedule<'_>,
    ) -> <C as BaseClient>::Response<SubscriptionSchedule> {
        client.post_form("/subscription_schedules", &params)
    }

    /// Retrieves the details of an existing subscription schedule.
    ///
    /// You only need to supply the unique subscription schedule identifier that was returned upon subscription schedule creation.
    pub fn retrieve<C: BaseClient>(
        client: &Client<C>,
        id: &SubscriptionScheduleId,
        expand: &[&str],
    ) -> <C as BaseClient>::Response<SubscriptionSchedule> {
        client.get_query(&format!("/subscription_schedules/{}", id), &Expand { expand })
    }

    /// Updates an existing subscription schedule.
    pub fn update<C: BaseClient>(
        client: &Client<C>,
        id: &SubscriptionScheduleId,
        params: UpdateSubscriptionSchedule<'_>,
    ) -> <C as BaseClient>::Response<SubscriptionSchedule> {
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionScheduleCurrentPhase {
    /// The end of this phase of the subscription schedule.
    pub end_date: Timestamp,

    /// The start of this phase of the subscription schedule.
    pub start_date: Timestamp,
}

/// The parameters for `SubscriptionSchedule::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateSubscriptionSchedule<'a> {
    /// The identifier of the customer to create the subscription schedule for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<SubscriptionScheduleDefaultSettingsParams>,

    /// Configures how the subscription schedule behaves when it ends.
    ///
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<SubscriptionScheduleEndBehavior>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Migrate an existing subscription to be managed by a subscription schedule.
    ///
    /// If this parameter is set, a subscription schedule will be created using the subscription's item(s), set to auto-renew using the subscription's interval.
    /// When using this parameter, other parameters (such as phase values) cannot be set.
    /// To create a subscription schedule with other modifications, we recommend making two separate API calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_subscription: Option<&'a str>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// List representing phases of the subscription schedule.
    ///
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<CreateSubscriptionSchedulePhases>>,

    /// When the subscription schedule starts.
    ///
    /// We recommend using `now` so that it starts the subscription immediately.
    /// You can also use a Unix timestamp to backdate the subscription so that it starts on a past date, or set a future date for the subscription to start on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Scheduled>,
}

impl<'a> CreateSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        CreateSubscriptionSchedule {
            customer: Default::default(),
            default_settings: Default::default(),
            end_behavior: Default::default(),
            expand: Default::default(),
            from_subscription: Default::default(),
            metadata: Default::default(),
            phases: Default::default(),
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
impl Paginable for ListSubscriptionSchedules<'_> {
    type O = SubscriptionSchedule;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `SubscriptionSchedule::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSubscriptionSchedule<'a> {
    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<SubscriptionScheduleDefaultSettingsParams>,

    /// Configures how the subscription schedule behaves when it ends.
    ///
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<SubscriptionScheduleEndBehavior>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// List representing phases of the subscription schedule.
    ///
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    /// Note that past phases can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<UpdateSubscriptionSchedulePhases>>,

    /// If the update changes the current phase, indicates whether the changes should be prorated.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<SubscriptionProrationBehavior>,
}

impl<'a> UpdateSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        UpdateSubscriptionSchedule {
            default_settings: Default::default(),
            end_behavior: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            phases: Default::default(),
            proration_behavior: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<AddInvoiceItems>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateSubscriptionSchedulePhasesAutomaticTax>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<CreateSubscriptionSchedulePhasesBillingCycleAnchor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    pub items: Vec<CreateSubscriptionSchedulePhasesItems>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<SubscriptionProrationBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateSubscriptionSchedulePhasesTransferData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionScheduleDefaultSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<SubscriptionScheduleDefaultSettingsParamsAutomaticTax>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<SubscriptionScheduleDefaultSettingsParamsBillingCycleAnchor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionScheduleBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<SubscriptionScheduleDefaultSettingsParamsTransferData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<AddInvoiceItems>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateSubscriptionSchedulePhasesAutomaticTax>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpdateSubscriptionSchedulePhasesBillingCycleAnchor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    pub items: Vec<UpdateSubscriptionSchedulePhasesItems>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<SubscriptionProrationBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdateSubscriptionSchedulePhasesTransferData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AddInvoiceItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<InvoiceItemPriceData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<CreateSubscriptionSchedulePhasesItemsBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateSubscriptionSchedulePhasesItemsPriceData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionScheduleBillingThresholds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionScheduleDefaultSettingsParamsAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionScheduleDefaultSettingsParamsTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionScheduleInvoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpdateSubscriptionSchedulePhasesItemsBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateSubscriptionSchedulePhasesItemsPriceData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsBillingThresholds {
    pub usage_gte: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsPriceData {
    pub currency: Currency,

    pub product: String,

    pub recurring: CreateSubscriptionSchedulePhasesItemsPriceDataRecurring,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceItemPriceData {
    pub currency: Currency,

    pub product: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<InvoiceItemPriceDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsBillingThresholds {
    pub usage_gte: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsPriceData {
    pub currency: Currency,

    pub product: String,

    pub recurring: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    pub interval: CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    pub interval: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

/// An enum representing the possible values of an `CreateSubscriptionSchedulePhases`'s `billing_cycle_anchor` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionSchedulePhasesBillingCycleAnchor::Automatic => "automatic",
            CreateSubscriptionSchedulePhasesBillingCycleAnchor::PhaseStart => "phase_start",
        }
    }
}

impl AsRef<str> for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateSubscriptionSchedulePhasesItemsPriceDataRecurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::Day => "day",
            CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::Month => "month",
            CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::Week => "week",
            CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::Year => "year",
        }
    }
}

impl AsRef<str> for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `CreateSubscriptionSchedulePhasesItemsPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `InvoiceItemPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl InvoiceItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceItemPriceDataTaxBehavior::Exclusive => "exclusive",
            InvoiceItemPriceDataTaxBehavior::Inclusive => "inclusive",
            InvoiceItemPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for InvoiceItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoiceItemPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionSchedule`'s `proration_behavior` field.
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
impl std::default::Default for SubscriptionProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

/// An enum representing the possible values of an `SubscriptionScheduleDefaultSettingsParams`'s `billing_cycle_anchor` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleDefaultSettingsParamsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl SubscriptionScheduleDefaultSettingsParamsBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionScheduleDefaultSettingsParamsBillingCycleAnchor::Automatic => "automatic",
            SubscriptionScheduleDefaultSettingsParamsBillingCycleAnchor::PhaseStart => {
                "phase_start"
            }
        }
    }
}

impl AsRef<str> for SubscriptionScheduleDefaultSettingsParamsBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleDefaultSettingsParamsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionScheduleDefaultSettingsParamsBillingCycleAnchor {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `SubscriptionSchedule`'s `end_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}

impl SubscriptionScheduleEndBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionScheduleEndBehavior::Cancel => "cancel",
            SubscriptionScheduleEndBehavior::None => "none",
            SubscriptionScheduleEndBehavior::Release => "release",
            SubscriptionScheduleEndBehavior::Renew => "renew",
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
impl std::default::Default for SubscriptionScheduleEndBehavior {
    fn default() -> Self {
        Self::Cancel
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
impl std::default::Default for SubscriptionScheduleStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionSchedulePhases`'s `billing_cycle_anchor` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionSchedulePhasesBillingCycleAnchor::Automatic => "automatic",
            UpdateSubscriptionSchedulePhasesBillingCycleAnchor::PhaseStart => "phase_start",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::Day => "day",
            UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::Month => "month",
            UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::Week => "week",
            UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::Year => "year",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionSchedulePhasesItemsPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::Exclusive => "exclusive",
            UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::Inclusive => "inclusive",
            UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
