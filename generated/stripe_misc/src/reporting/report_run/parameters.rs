#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Parameters {
    /// The set of output columns requested for inclusion in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// Connected account ID by which to filter the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account: Option<String>,
    /// Currency of objects to be included in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Ending timestamp of data to be included in the report run (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_end: Option<stripe_types::Timestamp>,
    /// Starting timestamp of data to be included in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_start: Option<stripe_types::Timestamp>,
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Parameters {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
