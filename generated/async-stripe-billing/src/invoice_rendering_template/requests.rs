use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListInvoiceRenderingTemplateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_billing::InvoiceRenderingTemplateStatus>,
}
impl ListInvoiceRenderingTemplateBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None, status: None }
    }
}
/// List all templates, ordered by creation date, with the most recently created template appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListInvoiceRenderingTemplate {
    inner: ListInvoiceRenderingTemplateBuilder,
}
impl ListInvoiceRenderingTemplate {
    /// Construct a new `ListInvoiceRenderingTemplate`.
    pub fn new() -> Self {
        Self { inner: ListInvoiceRenderingTemplateBuilder::new() }
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
    pub fn status(
        mut self,
        status: impl Into<stripe_billing::InvoiceRenderingTemplateStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListInvoiceRenderingTemplate {
    fn default() -> Self {
        Self::new()
    }
}
impl ListInvoiceRenderingTemplate {
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
        stripe_types::List<stripe_billing::InvoiceRenderingTemplate>,
    > {
        stripe_client_core::ListPaginator::new_list("/invoice_rendering_templates", &self.inner)
    }
}

impl StripeRequest for ListInvoiceRenderingTemplate {
    type Output = stripe_types::List<stripe_billing::InvoiceRenderingTemplate>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/invoice_rendering_templates").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveInvoiceRenderingTemplateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<i64>,
}
impl RetrieveInvoiceRenderingTemplateBuilder {
    fn new() -> Self {
        Self { expand: None, version: None }
    }
}
/// Retrieves an invoice rendering template with the given ID.
/// It by default returns the latest version of the template.
/// Optionally, specify a version to see previous versions.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveInvoiceRenderingTemplate {
    inner: RetrieveInvoiceRenderingTemplateBuilder,
    template: stripe_billing::InvoiceRenderingTemplateId,
}
impl RetrieveInvoiceRenderingTemplate {
    /// Construct a new `RetrieveInvoiceRenderingTemplate`.
    pub fn new(template: impl Into<stripe_billing::InvoiceRenderingTemplateId>) -> Self {
        Self { template: template.into(), inner: RetrieveInvoiceRenderingTemplateBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    pub fn version(mut self, version: impl Into<i64>) -> Self {
        self.inner.version = Some(version.into());
        self
    }
}
impl RetrieveInvoiceRenderingTemplate {
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

impl StripeRequest for RetrieveInvoiceRenderingTemplate {
    type Output = stripe_billing::InvoiceRenderingTemplate;

    fn build(&self) -> RequestBuilder {
        let template = &self.template;
        RequestBuilder::new(StripeMethod::Get, format!("/invoice_rendering_templates/{template}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ArchiveInvoiceRenderingTemplateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ArchiveInvoiceRenderingTemplateBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the status of an invoice rendering template to ‘archived’ so no new Stripe objects (customers, invoices, etc.) can reference it.
/// The template can also no longer be updated.
/// However, if the template is already set on a Stripe object, it will continue to be applied on invoices generated by it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ArchiveInvoiceRenderingTemplate {
    inner: ArchiveInvoiceRenderingTemplateBuilder,
    template: stripe_billing::InvoiceRenderingTemplateId,
}
impl ArchiveInvoiceRenderingTemplate {
    /// Construct a new `ArchiveInvoiceRenderingTemplate`.
    pub fn new(template: impl Into<stripe_billing::InvoiceRenderingTemplateId>) -> Self {
        Self { template: template.into(), inner: ArchiveInvoiceRenderingTemplateBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ArchiveInvoiceRenderingTemplate {
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

impl StripeRequest for ArchiveInvoiceRenderingTemplate {
    type Output = stripe_billing::InvoiceRenderingTemplate;

    fn build(&self) -> RequestBuilder {
        let template = &self.template;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/invoice_rendering_templates/{template}/archive"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct UnarchiveInvoiceRenderingTemplateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl UnarchiveInvoiceRenderingTemplateBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Unarchive an invoice rendering template so it can be used on new Stripe objects again.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UnarchiveInvoiceRenderingTemplate {
    inner: UnarchiveInvoiceRenderingTemplateBuilder,
    template: stripe_billing::InvoiceRenderingTemplateId,
}
impl UnarchiveInvoiceRenderingTemplate {
    /// Construct a new `UnarchiveInvoiceRenderingTemplate`.
    pub fn new(template: impl Into<stripe_billing::InvoiceRenderingTemplateId>) -> Self {
        Self { template: template.into(), inner: UnarchiveInvoiceRenderingTemplateBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl UnarchiveInvoiceRenderingTemplate {
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

impl StripeRequest for UnarchiveInvoiceRenderingTemplate {
    type Output = stripe_billing::InvoiceRenderingTemplate;

    fn build(&self) -> RequestBuilder {
        let template = &self.template;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/invoice_rendering_templates/{template}/unarchive"),
        )
        .form(&self.inner)
    }
}
