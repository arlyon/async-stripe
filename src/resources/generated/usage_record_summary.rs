// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::UsageRecordSummaryId;
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "UsageRecordSummary".
///
/// For more details see <https://stripe.com/docs/api/usage-record-summary/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UsageRecordSummary {
    /// Unique identifier for the object.
    pub id: UsageRecordSummaryId,

    /// The invoice in which this usage period has been billed for.
    pub invoice: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub period: Period,

    /// The ID of the subscription item this summary is describing.
    pub subscription_item: String,

    /// The total usage within this usage period.
    pub total_usage: i64,
}

impl Object for UsageRecordSummary {
    type Id = UsageRecordSummaryId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "usage_record_summary"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Period {
    /// The end date of this usage period.
    ///
    /// All usage up to and including this point in time is included.
    pub end: Option<Timestamp>,

    /// The start date of this usage period.
    ///
    /// All usage after this point in time is included.
    pub start: Option<Timestamp>,
}
