/// If you have [scheduled a Sigma query](https://stripe.com/docs/sigma/scheduled-queries), you'll
/// receive a `sigma.scheduled_query_run.created` webhook each time the query
/// runs.
///
/// The webhook contains a `ScheduledQueryRun` object, which you can use to retrieve the query results.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ScheduledQueryRun {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// When the query was run, Sigma contained a snapshot of your Stripe data at this time.
    pub data_load_time: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_misc::SigmaScheduledQueryRunError>,
    /// The file object representing the results of the query.
    pub file: Option<stripe_types::File>,
    /// Unique identifier for the object.
    pub id: stripe_misc::scheduled_query_run::ScheduledQueryRunId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Time at which the result expires and is no longer available for download.
    pub result_available_until: stripe_types::Timestamp,
    /// SQL for the query.
    pub sql: String,
    /// The query's execution status, which will be `completed` for successful runs, and `canceled`, `failed`, or `timed_out` otherwise.
    pub status: String,
    /// Title of the query.
    pub title: String,
}
impl stripe_types::Object for ScheduledQueryRun {
    type Id = stripe_misc::scheduled_query_run::ScheduledQueryRunId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ScheduledQueryRunId, "sqr_");
#[cfg(feature = "scheduled_query_run")]
mod requests;
#[cfg(feature = "scheduled_query_run")]
pub use requests::*;
