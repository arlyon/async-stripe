// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ReportingReportRunId};
use crate::params::{Object, Timestamp};
use crate::resources::{Currency, File};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "reporting_report_run".
///
/// For more details see <https://stripe.com/docs/api/reporting/report_run/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReportingReportRun {
    /// Unique identifier for the object.
    pub id: ReportingReportRunId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// If something should go wrong during the run, a message about the failure (populated when
    ///  `status=failed`).
    pub error: Option<String>,

    /// `true` if the report is run on live mode data and `false` if it is run on test mode data.
    pub livemode: bool,

    pub parameters: FinancialReportingFinanceReportRunRunParameters,

    /// The ID of the [report type](https://stripe.com/docs/reports/report-types) to run, such as `"balance.summary.1"`.
    pub report_type: String,

    /// The file object representing the result of the report run (populated when
    ///  `status=succeeded`).
    pub result: Option<File>,

    /// Status of this report run.
    ///
    /// This will be `pending` when the run is initially created.  When the run finishes, this will be set to `succeeded` and the `result` field will be populated.  Rarely, we may encounter an error, at which point this will be set to `failed` and the `error` field will be populated.
    pub status: String,

    /// Timestamp at which this run successfully finished (populated when
    ///  `status=succeeded`).
    ///
    /// Measured in seconds since the Unix epoch.
    pub succeeded_at: Option<Timestamp>,
}

impl Object for ReportingReportRun {
    type Id = ReportingReportRunId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "reporting.report_run"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialReportingFinanceReportRunRunParameters {

    /// The set of output columns requested for inclusion in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,

    /// Connected account ID by which to filter the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account: Option<String>,

    /// Currency of objects to be included in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Ending timestamp of data to be included in the report run.
    ///
    /// Can be any UTC timestamp between 1 second after the user specified `interval_start` and 1 second before this report's last `data_available_end` value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_end: Option<Timestamp>,

    /// Starting timestamp of data to be included in the report run.
    ///
    /// Can be any UTC timestamp between 1 second after this report's `data_available_start` and 1 second before the user specified `interval_end` value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_start: Option<Timestamp>,

    /// Payout ID by which to filter the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<String>,

    /// Category of balance transactions to be included in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_category: Option<String>,

    /// Defaults to `Etc/UTC`.
    ///
    /// The output timezone for all timestamps in the report.
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    /// Has no effect on `interval_start` or `interval_end`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}
