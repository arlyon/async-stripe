use params::Timestamp;
use resources::File;
use serde_json as json;

#[derive(Debug, Deserialize)]
pub struct ScheduledQueryRun {
    pub id: String,
    pub object: String,
    pub created: Timestamp,
    pub data_load_time: Timestamp,
    pub error: Option<json::Value>,
    pub file: File,
    pub livemode: bool,
    pub result_available_until: Timestamp,
    pub sql: String,
    pub status: String, // (completed, canceled, failed, timed_out)
    pub title: String,
}
