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
    pub created: stripe_types::Timestamp,
    /// When the query was run, Sigma contained a snapshot of your Stripe data at this time.
    pub data_load_time: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_misc::sigma::scheduled_query_run::run_error::RunError>,
    /// The file object representing the results of the query.
    pub file: Option<stripe_core::file::File>,
    /// Unique identifier for the object.
    pub id: stripe_misc::sigma::scheduled_query_run::ScheduledQueryRunId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ScheduledQueryRunObject,
    /// Time at which the result expires and is no longer available for download.
    pub result_available_until: stripe_types::Timestamp,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ScheduledQueryRunObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "scheduled_query_run" => Ok(Self::ScheduledQueryRun),

            _ => Err(()),
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
impl serde::Serialize for ScheduledQueryRunObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScheduledQueryRunObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ScheduledQueryRunObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ScheduledQueryRunObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ScheduledQueryRunObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ScheduledQueryRunObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for ScheduledQueryRun {
    type Id = stripe_misc::sigma::scheduled_query_run::ScheduledQueryRunId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ScheduledQueryRunId, "sqr_");
pub mod requests;
pub mod run_error;
pub use run_error::RunError;
