// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{BillingMeterEventSummaryId};
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingMeterEventSummary".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterEventSummary {
    /// Unique identifier for the object.
    pub id: BillingMeterEventSummaryId,

    /// Aggregated value of all the events within start_time (inclusive) and end_time (inclusive).
    ///
    /// The aggregation strategy is defined on meter via `default_aggregation``.
    pub aggregated_value: f64,

    /// End timestamp for this usage summary (inclusive).
    pub end_time: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The meter associated with this usage summary.
    pub meter: String,

    /// Start timestamp for this usage summary (inclusive).
    pub start_time: Timestamp,
}

impl Object for BillingMeterEventSummary {
    type Id = BillingMeterEventSummaryId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing.meter_event_summary"
    }
}
