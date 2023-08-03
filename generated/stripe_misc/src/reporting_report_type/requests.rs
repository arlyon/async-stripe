
/// Retrieves the details of a Report Type.
///
/// (Certain report types require a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).).
pub fn retrieve(client: &stripe::Client, report_type: &stripe_misc::reporting_report_type::ReportingReportTypeId, params: RetrieveReportingReportType) -> stripe::Response<stripe_misc::ReportingReportType> {
    client.get_query(&format!("/reporting/report_types/{report_type}", report_type = report_type), params)
}
/// Returns a full list of Report Types.
pub fn list(client: &stripe::Client, params: ListReportingReportType) -> stripe::Response<stripe_types::List<stripe_misc::ReportingReportType>> {
    client.get_query("/reporting/report_types", params)
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveReportingReportType<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveReportingReportType<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListReportingReportType<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ListReportingReportType<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
