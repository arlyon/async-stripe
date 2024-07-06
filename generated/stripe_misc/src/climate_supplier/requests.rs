use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListClimateSupplierBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListClimateSupplierBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Lists all available Climate supplier objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListClimateSupplier<'a> {
    inner: ListClimateSupplierBuilder<'a>,
}
impl<'a> ListClimateSupplier<'a> {
    /// Construct a new `ListClimateSupplier`.
    pub fn new() -> Self {
        Self { inner: ListClimateSupplierBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListClimateSupplier<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListClimateSupplier<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::ClimateSupplier>> {
        stripe_client_core::ListPaginator::new_list("/climate/suppliers", self.inner)
    }
}

impl StripeRequest for ListClimateSupplier<'_> {
    type Output = stripe_types::List<stripe_misc::ClimateSupplier>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/climate/suppliers").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveClimateSupplierBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveClimateSupplierBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Climate supplier object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveClimateSupplier<'a> {
    inner: RetrieveClimateSupplierBuilder<'a>,
    supplier: &'a stripe_misc::ClimateSupplierId,
}
impl<'a> RetrieveClimateSupplier<'a> {
    /// Construct a new `RetrieveClimateSupplier`.
    pub fn new(supplier: &'a stripe_misc::ClimateSupplierId) -> Self {
        Self { supplier, inner: RetrieveClimateSupplierBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveClimateSupplier<'_> {
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

impl StripeRequest for RetrieveClimateSupplier<'_> {
    type Output = stripe_misc::ClimateSupplier;

    fn build(&self) -> RequestBuilder {
        let supplier = self.supplier;
        RequestBuilder::new(StripeMethod::Get, format!("/climate/suppliers/{supplier}"))
            .query(&self.inner)
    }
}
