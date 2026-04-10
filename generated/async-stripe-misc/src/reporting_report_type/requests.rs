use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListReportingReportTypeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListReportingReportTypeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListReportingReportTypeBuilder").finish_non_exhaustive()
    }
}
impl ListReportingReportTypeBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Returns a full list of Report Types.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListReportingReportType {
    inner: ListReportingReportTypeBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListReportingReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListReportingReportType").finish_non_exhaustive()
    }
}
impl ListReportingReportType {
    /// Construct a new `ListReportingReportType`.
    pub fn new() -> Self {
        Self { inner: ListReportingReportTypeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl Default for ListReportingReportType {
    fn default() -> Self {
        Self::new()
    }
}
impl ListReportingReportType {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::ReportingReportType>>
    {
        stripe_client_core::ListPaginator::new_list("/reporting/report_types", &self.inner)
    }
}

impl StripeRequest for ListReportingReportType {
    type Output = stripe_types::List<stripe_misc::ReportingReportType>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/reporting/report_types").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveReportingReportTypeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveReportingReportTypeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveReportingReportTypeBuilder").finish_non_exhaustive()
    }
}
impl RetrieveReportingReportTypeBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a Report Type.
/// (Certain report types require a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveReportingReportType {
    inner: RetrieveReportingReportTypeBuilder,
    report_type: stripe_misc::ReportingReportTypeId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveReportingReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveReportingReportType").finish_non_exhaustive()
    }
}
impl RetrieveReportingReportType {
    /// Construct a new `RetrieveReportingReportType`.
    pub fn new(report_type: impl Into<stripe_misc::ReportingReportTypeId>) -> Self {
        Self { report_type: report_type.into(), inner: RetrieveReportingReportTypeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveReportingReportType {
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

impl StripeRequest for RetrieveReportingReportType {
    type Output = stripe_misc::ReportingReportType;

    fn build(&self) -> RequestBuilder {
        let report_type = &self.report_type;
        RequestBuilder::new(StripeMethod::Get, format!("/reporting/report_types/{report_type}"))
            .query(&self.inner)
    }
}
