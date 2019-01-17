use crate::params::{Identifiable, Timestamp};
use crate::resources::File;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe scheduled query run.
///
/// For more details see https://stripe.com/docs/api#scheduled_query_run_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScheduledQueryRun {
    pub id: String,
    pub object: String,
    pub created: Timestamp,
    pub data_load_time: Timestamp,
    pub error: Option<serde_json::Value>,
    pub file: File,
    pub livemode: bool,
    pub result_available_until: Timestamp,
    pub sql: String,
    pub status: String, // (completed, canceled, failed, timed_out)
    pub title: String,
}

impl Identifiable for ScheduledQueryRun {
    fn id(&self) -> &str {
        &self.id
    }
}
