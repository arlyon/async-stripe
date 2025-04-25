// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingMeterEventAdjustment".
///
/// For more details see <https://stripe.com/docs/api/billing/meter-event-adjustment/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterEventAdjustment {

    /// Specifies which event to cancel.
    pub cancel: Option<BillingMeterResourceBillingMeterEventAdjustmentCancel>,

    /// The name of the meter event.
    ///
    /// Corresponds with the `event_name` field on a meter.
    pub event_name: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The meter event adjustment's status.
    pub status: BillingMeterEventAdjustmentStatus,

    /// Specifies whether to cancel a single event or a range of events for a time period.
    ///
    /// Time period cancellation is not supported yet.
    #[serde(rename = "type")]
    pub type_: BillingMeterEventAdjustmentType,
}

impl Object for BillingMeterEventAdjustment {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "billing.meter_event_adjustment"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterResourceBillingMeterEventAdjustmentCancel {

    /// Unique identifier for the event.
    pub identifier: Option<String>,
}

/// An enum representing the possible values of an `BillingMeterEventAdjustment`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingMeterEventAdjustmentStatus {
    Complete,
    Pending,
}

impl BillingMeterEventAdjustmentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingMeterEventAdjustmentStatus::Complete => "complete",
            BillingMeterEventAdjustmentStatus::Pending => "pending",
        }
    }
}

impl AsRef<str> for BillingMeterEventAdjustmentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingMeterEventAdjustmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingMeterEventAdjustmentStatus {
    fn default() -> Self {
        Self::Complete
    }
}

/// An enum representing the possible values of an `BillingMeterEventAdjustment`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingMeterEventAdjustmentType {
    Cancel,
}

impl BillingMeterEventAdjustmentType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingMeterEventAdjustmentType::Cancel => "cancel",
        }
    }
}

impl AsRef<str> for BillingMeterEventAdjustmentType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingMeterEventAdjustmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingMeterEventAdjustmentType {
    fn default() -> Self {
        Self::Cancel
    }
}
