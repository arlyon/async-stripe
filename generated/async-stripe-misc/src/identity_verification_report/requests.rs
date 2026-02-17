use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListIdentityVerificationReportBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<ListIdentityVerificationReportType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_session: Option<String>,
}
impl ListIdentityVerificationReportBuilder {
    fn new() -> Self {
        Self {
            client_reference_id: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
            verification_session: None,
        }
    }
}
/// Only return VerificationReports of this type
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListIdentityVerificationReportType {
    Document,
    IdNumber,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListIdentityVerificationReportType {
    pub fn as_str(&self) -> &str {
        use ListIdentityVerificationReportType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListIdentityVerificationReportType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListIdentityVerificationReportType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ListIdentityVerificationReportType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ListIdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListIdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListIdentityVerificationReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListIdentityVerificationReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// List all verification reports.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIdentityVerificationReport {
    inner: ListIdentityVerificationReportBuilder,
}
impl ListIdentityVerificationReport {
    /// Construct a new `ListIdentityVerificationReport`.
    pub fn new() -> Self {
        Self { inner: ListIdentityVerificationReportBuilder::new() }
    }
    /// A string to reference this user.
    /// This can be a customer ID, a session ID, or similar, and can be used to reconcile this verification with your internal systems.
    pub fn client_reference_id(mut self, client_reference_id: impl Into<String>) -> Self {
        self.inner.client_reference_id = Some(client_reference_id.into());
        self
    }
    /// Only return VerificationReports that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return VerificationReports of this type
    pub fn type_(mut self, type_: impl Into<ListIdentityVerificationReportType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
    /// Only return VerificationReports created by this VerificationSession ID.
    /// It is allowed to provide a VerificationIntent ID.
    pub fn verification_session(mut self, verification_session: impl Into<String>) -> Self {
        self.inner.verification_session = Some(verification_session.into());
        self
    }
}
impl Default for ListIdentityVerificationReport {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIdentityVerificationReport {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_misc::IdentityVerificationReport>,
    > {
        stripe_client_core::ListPaginator::new_list("/identity/verification_reports", &self.inner)
    }
}

impl StripeRequest for ListIdentityVerificationReport {
    type Output = stripe_types::List<stripe_misc::IdentityVerificationReport>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/identity/verification_reports").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveIdentityVerificationReportBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveIdentityVerificationReportBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an existing VerificationReport
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveIdentityVerificationReport {
    inner: RetrieveIdentityVerificationReportBuilder,
    report: stripe_misc::IdentityVerificationReportId,
}
impl RetrieveIdentityVerificationReport {
    /// Construct a new `RetrieveIdentityVerificationReport`.
    pub fn new(report: impl Into<stripe_misc::IdentityVerificationReportId>) -> Self {
        Self { report: report.into(), inner: RetrieveIdentityVerificationReportBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIdentityVerificationReport {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveIdentityVerificationReport {
    type Output = stripe_misc::IdentityVerificationReport;

    fn build(&self) -> RequestBuilder {
        let report = &self.report;
        RequestBuilder::new(StripeMethod::Get, format!("/identity/verification_reports/{report}"))
            .query(&self.inner)
    }
}
