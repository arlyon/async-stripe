#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveGelatoVerificationReport<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveGelatoVerificationReport<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveGelatoVerificationReport<'a> {
    /// Retrieves an existing VerificationReport.
    pub fn send(
        &self,
        client: &stripe::Client,
        report: &stripe_misc::gelato_verification_report::IdentityVerificationReportId,
    ) -> stripe::Response<stripe_misc::GelatoVerificationReport> {
        client.get_query(&format!("/identity/verification_reports/{report}", report = report), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListGelatoVerificationReport<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
    /// Only return VerificationReports of this type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ListGelatoVerificationReportType>,
    /// Only return VerificationReports created by this VerificationSession ID.
    ///
    /// It is allowed to provide a VerificationIntent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<&'a str>,
}
impl<'a> ListGelatoVerificationReport<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return VerificationReports of this type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListGelatoVerificationReportType {
    Document,
    IdNumber,
}

impl ListGelatoVerificationReportType {
    pub fn as_str(self) -> &'static str {
        use ListGelatoVerificationReportType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for ListGelatoVerificationReportType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListGelatoVerificationReportType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListGelatoVerificationReportType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListGelatoVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListGelatoVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListGelatoVerificationReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> stripe::PaginationParams for ListGelatoVerificationReport<'a> {}
impl<'a> ListGelatoVerificationReport<'a> {
    /// List all verification reports.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::GelatoVerificationReport>> {
        client.get_query("/identity/verification_reports", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_misc::GelatoVerificationReport> {
        stripe::ListPaginator::from_params("/identity/verification_reports", self)
    }
}
