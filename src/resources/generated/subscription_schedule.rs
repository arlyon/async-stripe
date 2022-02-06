// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::CustomerId;
use crate::params::{Expand, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{CollectionMethod, Currency, Scheduled, SubscriptionBillingThresholds};
use crate::{AddInvoiceItems, SubscriptionProrationBehavior, SubscriptionSchedule};

impl SubscriptionSchedule {
    /// Retrieves the list of your subscription schedules.
    pub fn list(
        client: &Client,
        params: ListSubscriptionSchedules<'_>,
    ) -> Response<List<SubscriptionSchedule>> {
        client.get_query("/subscription_schedules", &params)
    }

    /// Creates a new subscription schedule object.
    ///
    /// Each customer can have up to 500 active or scheduled subscriptions.
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

// written at 597
/// The parameters for `SubscriptionSchedule::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateSubscriptionSchedule<'a> {
    /// The identifier of the customer to create the subscription schedule for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<Box<SubscriptionScheduleDefaultSettingsParams>>,

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
    pub phases: Option<Box<Vec<CreateSubscriptionSchedulePhases>>>,

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

// written at 597
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

// written at 597
/// The parameters for `SubscriptionSchedule::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSubscriptionSchedule<'a> {
    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<Box<SubscriptionScheduleDefaultSettingsParams>>,

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
    pub phases: Option<Box<Vec<UpdateSubscriptionSchedulePhases>>>,

    /// If the update changes the current phase, indicates if the changes should be prorated.
    ///
    /// Possible values are `create_prorations` or `none`, and the default value is `create_prorations`.
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

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Box<Vec<AddInvoiceItems>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<Box<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<Box<CreateSubscriptionSchedulePhasesAutomaticTax>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<Box<CreateSubscriptionSchedulePhasesBillingCycleAnchor>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Box<Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<Box<SubscriptionScheduleInvoiceSettings>>,

    pub items: Vec<CreateSubscriptionSchedulePhasesItems>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<Box<SubscriptionProrationBehavior>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<CreateSubscriptionSchedulePhasesTransferData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleDefaultSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<Box<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<Box<SubscriptionScheduleDefaultSettingsParamsAutomaticTax>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor:
        Option<Box<SubscriptionScheduleDefaultSettingsParamsBillingCycleAnchor>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<Box<SubscriptionScheduleBillingThresholds>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<SubscriptionScheduleDefaultSettingsParamsTransferData>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Box<Vec<AddInvoiceItems>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<Box<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<Box<UpdateSubscriptionSchedulePhasesAutomaticTax>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<Box<UpdateSubscriptionSchedulePhasesBillingCycleAnchor>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Box<Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<Box<SubscriptionScheduleInvoiceSettings>>,

    pub items: Vec<UpdateSubscriptionSchedulePhasesItems>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<Box<SubscriptionProrationBehavior>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<UpdateSubscriptionSchedulePhasesTransferData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesAutomaticTax {
    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<Box<CreateSubscriptionSchedulePhasesItemsBillingThresholds>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<Box<CreateSubscriptionSchedulePhasesItemsPriceData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<Box<f64>>,

    pub destination: String,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleBillingThresholds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleDefaultSettingsParamsAutomaticTax {
    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleDefaultSettingsParamsTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<Box<f64>>,

    pub destination: String,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleInvoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<Box<u32>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesAutomaticTax {
    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<Box<UpdateSubscriptionSchedulePhasesItemsBillingThresholds>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<Box<UpdateSubscriptionSchedulePhasesItemsPriceData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<Box<f64>>,

    pub destination: String,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsBillingThresholds {
    pub usage_gte: i64,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsPriceData {
    pub currency: Currency,

    pub product: String,

    pub recurring: CreateSubscriptionSchedulePhasesItemsPriceDataRecurring,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<Box<CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsBillingThresholds {
    pub usage_gte: i64,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsPriceData {
    pub currency: Currency,

    pub product: String,

    pub recurring: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<Box<UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    pub interval: CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<Box<u64>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    pub interval: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<Box<u64>>,
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

/// An enum representing the possible values of an `CreateSubscriptionSchedule`'s `end_behavior` field.
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
