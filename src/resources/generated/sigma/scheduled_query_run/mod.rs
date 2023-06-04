/// If you have [scheduled a Sigma query](https://stripe.com/docs/sigma/scheduled-queries), you'll
/// receive a `sigma.scheduled_query_run.created` webhook each time the query
/// runs.
///
/// The webhook contains a `ScheduledQueryRun` object, which you can use to retrieve the query results.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ScheduledQueryRun {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// When the query was run, Sigma contained a snapshot of your Stripe data at this time.
    pub data_load_time: crate::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::sigma::scheduled_query_run::run_error::RunError>,
    /// The file object representing the results of the query.
    pub file: Option<crate::file::File>,
    /// Unique identifier for the object.
    pub id: crate::sigma::scheduled_query_run::ScheduledQueryRunId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ScheduledQueryRunObject,
    /// Time at which the result expires and is no longer available for download.
    pub result_available_until: crate::Timestamp,
    /// SQL for the query.
    pub sql: String,
    /// The query's execution status, which will be `completed` for successful runs, and `canceled`, `failed`, or `timed_out` otherwise.
    pub status: String,
    /// Title of the query.
    pub title: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ScheduledQueryRun {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ScheduledQueryRunObject {
    ScheduledQueryRun,
}

impl ScheduledQueryRunObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ScheduledQueryRun => "scheduled_query_run",
        }
    }
}

impl AsRef<str> for ScheduledQueryRunObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ScheduledQueryRunObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for ScheduledQueryRun {
    type Id = crate::sigma::scheduled_query_run::ScheduledQueryRunId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(ScheduledQueryRunId, "sqr_");
pub mod requests;
pub mod run_error;
pub use run_error::RunError;
