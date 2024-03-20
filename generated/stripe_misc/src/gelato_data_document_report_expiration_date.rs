/// Point in Time
#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GelatoDataDocumentReportExpirationDate {
    /// Numerical day between 1 and 31.
    pub day: Option<i64>,
    /// Numerical month between 1 and 12.
    pub month: Option<i64>,
    /// The four-digit year.
    pub year: Option<i64>,
}
