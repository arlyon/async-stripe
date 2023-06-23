/// The Report Run object represents an instance of a report type generated with
/// specific run parameters.
///
/// Once the object is created, Stripe begins processing the report. When the report has finished running, it will give you a reference to a file where you can retrieve your results.
/// For an overview, see [API Access to Reports](https://stripe.com/docs/reporting/statements/api).  Note that certain report types can only be run based on your live-mode data (not test-mode data), and will error when queried without a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReportRun {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If something should go wrong during the run, a message about the failure (populated when
    ///  `status=failed`).
    pub error: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::reporting::report_run::ReportingReportRunId,
    /// `true` if the report is run on live mode data and `false` if it is run on test mode data.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ReportRunObject,
    pub parameters: stripe_misc::reporting::report_run::parameters::Parameters,
    /// The ID of the [report type](https://stripe.com/docs/reports/report-types) to run, such as `"balance.summary.1"`.
    pub report_type: String,
    /// The file object representing the result of the report run (populated when
    ///  `status=succeeded`).
    pub result: Option<stripe_core::file::File>,
    /// Status of this report run.
    ///
    /// This will be `pending` when the run is initially created.  When the run finishes, this will be set to `succeeded` and the `result` field will be populated.  Rarely, we may encounter an error, at which point this will be set to `failed` and the `error` field will be populated.
    pub status: String,
    /// Timestamp at which this run successfully finished (populated when
    ///  `status=succeeded`).
    ///
    /// Measured in seconds since the Unix epoch.
    pub succeeded_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReportRun {
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
pub enum ReportRunObject {
    #[serde(rename = "reporting.report_run")]
    ReportingReportRun,
}

impl ReportRunObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReportingReportRun => "reporting.report_run",
        }
    }
}

impl AsRef<str> for ReportRunObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReportRunObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for ReportRun {
    type Id = stripe_misc::reporting::report_run::ReportingReportRunId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ReportingReportRunId);
pub mod parameters;
pub mod requests;
pub use parameters::Parameters;
