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
impl<'a> ListReportingReportType<'a> {
    /// Returns a full list of Report Types.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::ReportingReportType>> {
        client.get_query("/reporting/report_types", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::ReportingReportType>> {
        stripe::ListPaginator::from_list_params("/reporting/report_types", self)
    }
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
impl<'a> RetrieveReportingReportType<'a> {
    /// Retrieves the details of a Report Type.
    /// (Certain report types require a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).).
    pub fn send(
        &self,
        client: &stripe::Client,
        report_type: &stripe_misc::ReportingReportTypeId,
    ) -> stripe::Response<stripe_misc::ReportingReportType> {
        client.get_query(&format!("/reporting/report_types/{report_type}"), self)
    }
}
