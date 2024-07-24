use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListIssuingPhysicalBundleBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::IssuingPhysicalBundleStatus>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::IssuingPhysicalBundleType>,
}
impl<'a> ListIssuingPhysicalBundleBuilder<'a> {
    fn new() -> Self {
        Self {
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
            type_: None,
        }
    }
}
/// Returns a list of physical bundle objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIssuingPhysicalBundle<'a> {
    inner: ListIssuingPhysicalBundleBuilder<'a>,
}
impl<'a> ListIssuingPhysicalBundle<'a> {
    /// Construct a new `ListIssuingPhysicalBundle`.
    pub fn new() -> Self {
        Self { inner: ListIssuingPhysicalBundleBuilder::new() }
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
    /// Only return physical bundles with the given status.
    pub fn status(mut self, status: stripe_shared::IssuingPhysicalBundleStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
    /// Only return physical bundles with the given type.
    pub fn type_(mut self, type_: stripe_shared::IssuingPhysicalBundleType) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
}
impl<'a> Default for ListIssuingPhysicalBundle<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIssuingPhysicalBundle<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::IssuingPhysicalBundle>>
    {
        stripe_client_core::ListPaginator::new_list("/issuing/physical_bundles", self.inner)
    }
}

impl StripeRequest for ListIssuingPhysicalBundle<'_> {
    type Output = stripe_types::List<stripe_shared::IssuingPhysicalBundle>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/physical_bundles").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveIssuingPhysicalBundleBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIssuingPhysicalBundleBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a physical bundle object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveIssuingPhysicalBundle<'a> {
    inner: RetrieveIssuingPhysicalBundleBuilder<'a>,
    physical_bundle: &'a stripe_shared::IssuingPhysicalBundleId,
}
impl<'a> RetrieveIssuingPhysicalBundle<'a> {
    /// Construct a new `RetrieveIssuingPhysicalBundle`.
    pub fn new(physical_bundle: &'a stripe_shared::IssuingPhysicalBundleId) -> Self {
        Self { physical_bundle, inner: RetrieveIssuingPhysicalBundleBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveIssuingPhysicalBundle<'_> {
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

impl StripeRequest for RetrieveIssuingPhysicalBundle<'_> {
    type Output = stripe_shared::IssuingPhysicalBundle;

    fn build(&self) -> RequestBuilder {
        let physical_bundle = self.physical_bundle;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/issuing/physical_bundles/{physical_bundle}"),
        )
        .query(&self.inner)
    }
}
