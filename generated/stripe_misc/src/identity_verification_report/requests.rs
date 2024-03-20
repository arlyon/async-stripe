#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListIdentityVerificationReport<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return VerificationReports of this type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_misc::IdentityVerificationReportType>,
    /// Only return VerificationReports created by this VerificationSession ID.
    /// It is allowed to provide a VerificationIntent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<&'a str>,
}
impl<'a> ListIdentityVerificationReport<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListIdentityVerificationReport<'a> {
    /// List all verification reports.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::IdentityVerificationReport>> {
        client.get_query("/identity/verification_reports", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::IdentityVerificationReport>> {
        stripe::ListPaginator::from_list_params("/identity/verification_reports", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveIdentityVerificationReport<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIdentityVerificationReport<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveIdentityVerificationReport<'a> {
    /// Retrieves an existing VerificationReport
    pub fn send(
        &self,
        client: &stripe::Client,
        report: &stripe_misc::IdentityVerificationReportId,
    ) -> stripe::Response<stripe_misc::IdentityVerificationReport> {
        client.get_query(&format!("/identity/verification_reports/{report}"), self)
    }
}
