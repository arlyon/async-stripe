/// The Report Run object represents an instance of a report type generated with
/// specific run parameters.
///
/// Once the object is created, Stripe begins processing the report. When the report has finished running, it will give you a reference to a file where you can retrieve your results.
/// For an overview, see [API Access to Reports](https://stripe.com/docs/reporting/statements/api).  Note that certain report types can only be run based on your live-mode data (not test-mode data), and will error when queried without a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReportingReportRun {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If something should go wrong during the run, a message about the failure (populated when
    ///  `status=failed`).
    pub error: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::reporting_report_run::ReportingReportRunId,
    /// `true` if the report is run on live mode data and `false` if it is run on test mode data.
    pub livemode: bool,
    pub parameters: stripe_misc::FinancialReportingFinanceReportRunRunParameters,
    /// The ID of the [report type](https://stripe.com/docs/reports/report-types) to run, such as `"balance.summary.1"`.
    pub report_type: String,
    /// The file object representing the result of the report run (populated when
    ///  `status=succeeded`).
    pub result: Option<stripe_types::File>,
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
impl stripe_types::Object for ReportingReportRun {
    type Id = stripe_misc::reporting_report_run::ReportingReportRunId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(ReportingReportRunId);
#[cfg(feature = "reporting_report_run")]
mod requests;
#[cfg(feature = "reporting_report_run")]
pub use requests::*;
