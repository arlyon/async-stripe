/// The Report Type resource corresponds to a particular type of report, such as
/// the "Activity summary" or "Itemized payouts" reports.
///
/// These objects are identified by an ID belonging to a set of enumerated values.
/// See [API Access to Reports documentation](https://stripe.com/docs/reporting/statements/api) for those Report Type IDs, along with required and optional parameters.  Note that certain report types can only be run based on your live-mode data (not test-mode data), and will error when queried without a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReportType {
    /// Most recent time for which this Report Type is available.
    ///
    /// Measured in seconds since the Unix epoch.
    pub data_available_end: crate::Timestamp,
    /// Earliest time for which this Report Type is available.
    ///
    /// Measured in seconds since the Unix epoch.
    pub data_available_start: crate::Timestamp,
    /// List of column names that are included by default when this Report Type gets run.
    ///
    /// (If the Report Type doesn't support the `columns` parameter, this will be null.).
    pub default_columns: Option<Vec<String>>,
    /// The [ID of the Report Type](https://stripe.com/docs/reporting/statements/api#available-report-types), such as `balance.summary.1`.
    pub id: crate::reporting::report_type::ReportingReportTypeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Human-readable name of the Report Type.
    pub name: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ReportTypeObject,
    /// When this Report Type was latest updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: crate::Timestamp,
    /// Version of the Report Type.
    ///
    /// Different versions report with the same ID will have the same purpose, but may take different run parameters or have different result schemas.
    pub version: i64,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReportType {
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
pub enum ReportTypeObject {
    #[serde(rename = "reporting.report_type")]
    ReportingReportType,
}

impl ReportTypeObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReportingReportType => "reporting.report_type",
        }
    }
}

impl AsRef<str> for ReportTypeObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReportTypeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for ReportType {
    type Id = crate::reporting::report_type::ReportingReportTypeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(ReportingReportTypeId);
pub mod requests;
