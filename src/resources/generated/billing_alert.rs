// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{BillingAlertId};
use crate::params::{Expandable, Object};
use crate::resources::{BillingMeter, Customer};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ThresholdsResourceAlert".
///
/// For more details see <https://stripe.com/docs/api/billing/alert/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingAlert {
    /// Unique identifier for the object.
    pub id: BillingAlertId,

    /// Defines the type of the alert.
    pub alert_type: BillingAlertAlertType,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Status of the alert.
    ///
    /// This can be active, inactive or archived.
    pub status: Option<BillingAlertStatus>,

    /// Title of the alert.
    pub title: String,

    /// Encapsulates configuration of the alert to monitor usage on a specific [Billing Meter](https://stripe.com/docs/api/billing/meter).
    pub usage_threshold: Option<ThresholdsResourceUsageThresholdConfig>,
}

impl Object for BillingAlert {
    type Id = BillingAlertId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing.alert"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ThresholdsResourceUsageThresholdConfig {

    /// The filters allow limiting the scope of this usage alert.
    ///
    /// You can only specify up to one filter at this time.
    pub filters: Option<Vec<ThresholdsResourceUsageAlertFilter>>,

    /// The value at which this alert will trigger.
    pub gte: i64,

    /// The [Billing Meter](/api/billing/meter) ID whose usage is monitored.
    pub meter: Expandable<BillingMeter>,

    /// Defines how the alert will behave.
    pub recurrence: ThresholdsResourceUsageThresholdConfigRecurrence,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ThresholdsResourceUsageAlertFilter {

    /// Limit the scope of the alert to this customer ID.
    pub customer: Option<Expandable<Customer>>,

    #[serde(rename = "type")]
    pub type_: ThresholdsResourceUsageAlertFilterType,
}

/// An enum representing the possible values of an `BillingAlert`'s `alert_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingAlertAlertType {
    UsageThreshold,
}

impl BillingAlertAlertType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingAlertAlertType::UsageThreshold => "usage_threshold",
        }
    }
}

impl AsRef<str> for BillingAlertAlertType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingAlertAlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingAlertAlertType {
    fn default() -> Self {
        Self::UsageThreshold
    }
}

/// An enum representing the possible values of an `BillingAlert`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingAlertStatus {
    Active,
    Archived,
    Inactive,
}

impl BillingAlertStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingAlertStatus::Active => "active",
            BillingAlertStatus::Archived => "archived",
            BillingAlertStatus::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for BillingAlertStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingAlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingAlertStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `ThresholdsResourceUsageAlertFilter`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdsResourceUsageAlertFilterType {
    Customer,
}

impl ThresholdsResourceUsageAlertFilterType {
    pub fn as_str(self) -> &'static str {
        match self {
            ThresholdsResourceUsageAlertFilterType::Customer => "customer",
        }
    }
}

impl AsRef<str> for ThresholdsResourceUsageAlertFilterType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThresholdsResourceUsageAlertFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThresholdsResourceUsageAlertFilterType {
    fn default() -> Self {
        Self::Customer
    }
}

/// An enum representing the possible values of an `ThresholdsResourceUsageThresholdConfig`'s `recurrence` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdsResourceUsageThresholdConfigRecurrence {
    OneTime,
}

impl ThresholdsResourceUsageThresholdConfigRecurrence {
    pub fn as_str(self) -> &'static str {
        match self {
            ThresholdsResourceUsageThresholdConfigRecurrence::OneTime => "one_time",
        }
    }
}

impl AsRef<str> for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn default() -> Self {
        Self::OneTime
    }
}
