use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListReportingReportTypeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> ListReportingReportTypeBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Returns a full list of Report Types.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListReportingReportType<'a> {
    inner: ListReportingReportTypeBuilder<'a>,
}
impl<'a> ListReportingReportType<'a> {
    /// Construct a new `ListReportingReportType`.
    pub fn new() -> Self {
        Self { inner: ListReportingReportTypeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl<'a> Default for ListReportingReportType<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListReportingReportType<'_> {
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
        stripe_client_core::ListPaginator::new_list("/reporting/report_types", self.inner)
    }
}

impl StripeRequest for ListReportingReportType<'_> {
    type Output = stripe_types::List<stripe_misc::ReportingReportType>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/reporting/report_types").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveReportingReportTypeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveReportingReportTypeBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a Report Type.
/// (Certain report types require a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).).
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveReportingReportType<'a> {
    inner: RetrieveReportingReportTypeBuilder<'a>,
    report_type: &'a stripe_misc::ReportingReportTypeId,
}
impl<'a> RetrieveReportingReportType<'a> {
    /// Construct a new `RetrieveReportingReportType`.
    pub fn new(report_type: &'a stripe_misc::ReportingReportTypeId) -> Self {
        Self { report_type, inner: RetrieveReportingReportTypeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveReportingReportType<'_> {
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

impl StripeRequest for RetrieveReportingReportType<'_> {
    type Output = stripe_misc::ReportingReportType;

    fn build(&self) -> RequestBuilder {
        let report_type = self.report_type;
        RequestBuilder::new(StripeMethod::Get, format!("/reporting/report_types/{report_type}"))
            .query(&self.inner)
    }
}
