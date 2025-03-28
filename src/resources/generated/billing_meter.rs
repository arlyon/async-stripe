// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{BillingMeterId};
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingMeter".
///
/// For more details see <https://stripe.com/docs/api/billing/meter/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeter {
    /// Unique identifier for the object.
    pub id: BillingMeterId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    pub customer_mapping: BillingMeterResourceCustomerMappingSettings,

    pub default_aggregation: BillingMeterResourceAggregationSettings,

    /// The meter's name.
    pub display_name: String,

    /// The name of the meter event to record usage for.
    ///
    /// Corresponds with the `event_name` field on meter events.
    pub event_name: String,

    /// The time window to pre-aggregate meter events for, if any.
    pub event_time_window: Option<BillingMeterEventTimeWindow>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The meter's status.
    pub status: BillingMeterStatus,

    pub status_transitions: BillingMeterResourceBillingMeterStatusTransitions,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: Timestamp,

    pub value_settings: BillingMeterResourceBillingMeterValue,
}

impl Object for BillingMeter {
    type Id = BillingMeterId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing.meter"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterResourceAggregationSettings {

    /// Specifies how events are aggregated.
    pub formula: BillingMeterResourceAggregationSettingsFormula,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterResourceBillingMeterStatusTransitions {

    /// The time the meter was deactivated, if any.
    ///
    /// Measured in seconds since Unix epoch.
    pub deactivated_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterResourceBillingMeterValue {

    /// The key in the meter event payload to use as the value for this meter.
    pub event_payload_key: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterResourceCustomerMappingSettings {

    /// The key in the meter event payload to use for mapping the event to a customer.
    pub event_payload_key: String,

    /// The method for mapping a meter event to a customer.
    #[serde(rename = "type")]
    pub type_: BillingMeterResourceCustomerMappingSettingsType,
}

/// An enum representing the possible values of an `BillingMeter`'s `event_time_window` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingMeterEventTimeWindow {
    Day,
    Hour,
}

impl BillingMeterEventTimeWindow {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingMeterEventTimeWindow::Day => "day",
            BillingMeterEventTimeWindow::Hour => "hour",
        }
    }
}

impl AsRef<str> for BillingMeterEventTimeWindow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingMeterEventTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingMeterEventTimeWindow {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `BillingMeterResourceAggregationSettings`'s `formula` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingMeterResourceAggregationSettingsFormula {
    Count,
    Last,
    Sum,
}

impl BillingMeterResourceAggregationSettingsFormula {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingMeterResourceAggregationSettingsFormula::Count => "count",
            BillingMeterResourceAggregationSettingsFormula::Last => "last",
            BillingMeterResourceAggregationSettingsFormula::Sum => "sum",
        }
    }
}

impl AsRef<str> for BillingMeterResourceAggregationSettingsFormula {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingMeterResourceAggregationSettingsFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingMeterResourceAggregationSettingsFormula {
    fn default() -> Self {
        Self::Count
    }
}

/// An enum representing the possible values of an `BillingMeterResourceCustomerMappingSettings`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingMeterResourceCustomerMappingSettingsType {
    ById,
}

impl BillingMeterResourceCustomerMappingSettingsType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingMeterResourceCustomerMappingSettingsType::ById => "by_id",
        }
    }
}

impl AsRef<str> for BillingMeterResourceCustomerMappingSettingsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingMeterResourceCustomerMappingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingMeterResourceCustomerMappingSettingsType {
    fn default() -> Self {
        Self::ById
    }
}

/// An enum representing the possible values of an `BillingMeter`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingMeterStatus {
    Active,
    Inactive,
}

impl BillingMeterStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingMeterStatus::Active => "active",
            BillingMeterStatus::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for BillingMeterStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingMeterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingMeterStatus {
    fn default() -> Self {
        Self::Active
    }
}
