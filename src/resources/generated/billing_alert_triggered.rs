// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object, Timestamp};
use crate::resources::{BillingAlert};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ThresholdsResourceAlert".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingAlertTriggered {

    pub alert: BillingAlert,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// ID of customer for which the alert triggered.
    pub customer: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The value triggering the alert.
    pub value: i64,
}

impl Object for BillingAlertTriggered {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "billing.alert_triggered"
    }
}
