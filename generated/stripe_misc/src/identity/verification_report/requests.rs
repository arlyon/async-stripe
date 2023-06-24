use stripe::{Client, Response};

impl stripe_misc::identity::verification_report::VerificationReport {
    /// Retrieves an existing VerificationReport.
    pub fn retrieve(
        client: &Client,
        report: &str,
        params: RetrieveVerificationReport,
    ) -> Response<stripe_misc::identity::verification_report::VerificationReport> {
        client
            .get_query(&format!("/identity/verification_reports/{report}", report = report), params)
    }
    /// List all verification reports.
    pub fn list(
        client: &Client,
        params: ListVerificationReport,
    ) -> Response<stripe_types::List<stripe_misc::identity::verification_report::VerificationReport>>
    {
        client.get_query("/identity/verification_reports", params)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveVerificationReport<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveVerificationReport<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListVerificationReport<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return VerificationReports of this type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ListVerificationReportType>,
    /// Only return VerificationReports created by this VerificationSession ID.
    ///
    /// It is allowed to provide a VerificationIntent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<&'a str>,
}
impl<'a> ListVerificationReport<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return VerificationReports of this type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ListVerificationReportType {
    Document,
    IdNumber,
}

impl ListVerificationReportType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for ListVerificationReportType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "document" => Ok(Self::Document),
            "id_number" => Ok(Self::IdNumber),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListVerificationReportType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ListVerificationReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
