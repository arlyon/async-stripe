// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingMeterEventAdjustment".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterEventAdjustment {

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The meter event adjustment's status.
    pub status: BillingMeterEventAdjustmentStatus,
}

impl Object for BillingMeterEventAdjustment {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "billing.meter_event_adjustment"
    }
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
