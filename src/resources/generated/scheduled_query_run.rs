// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::ScheduledQueryRunId;
use crate::params::{Object, Timestamp};
use crate::resources::File;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ScheduledQueryRun".
///
/// For more details see <https://stripe.com/docs/api/sigma/scheduled_queries/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ScheduledQueryRun {
    /// Unique identifier for the object.
    pub id: ScheduledQueryRunId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// When the query was run, Sigma contained a snapshot of your Stripe data at this time.
    pub data_load_time: Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<SigmaScheduledQueryRunError>,

    /// The file object representing the results of the query.
    pub file: Option<File>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Time at which the result expires and is no longer available for download.
    pub result_available_until: Timestamp,

    /// SQL for the query.
    pub sql: String,

    /// The query's execution status, which will be `completed` for successful runs, and `canceled`, `failed`, or `timed_out` otherwise.
    pub status: String,

    /// Title of the query.
    pub title: String,
}

impl Object for ScheduledQueryRun {
    type Id = ScheduledQueryRunId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "scheduled_query_run"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SigmaScheduledQueryRunError {
    /// Information about the run failure.
    pub message: String,
}
