use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListEntitlementsFeatureBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListEntitlementsFeatureBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Retrieve a list of features
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListEntitlementsFeature<'a> {
    inner: ListEntitlementsFeatureBuilder<'a>,
}
impl<'a> ListEntitlementsFeature<'a> {
    /// Construct a new `ListEntitlementsFeature`.
    pub fn new() -> Self {
        Self { inner: ListEntitlementsFeatureBuilder::new() }
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
impl<'a> Default for ListEntitlementsFeature<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListEntitlementsFeature<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::EntitlementsFeature>>
    {
        stripe_client_core::ListPaginator::new_list("/entitlements/features", self.inner)
    }
}

impl StripeRequest for ListEntitlementsFeature<'_> {
    type Output = stripe_types::List<stripe_shared::EntitlementsFeature>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/entitlements/features").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveEntitlementsFeatureBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveEntitlementsFeatureBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a feature
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveEntitlementsFeature<'a> {
    inner: RetrieveEntitlementsFeatureBuilder<'a>,
    id: &'a stripe_shared::EntitlementsFeatureId,
}
impl<'a> RetrieveEntitlementsFeature<'a> {
    /// Construct a new `RetrieveEntitlementsFeature`.
    pub fn new(id: &'a stripe_shared::EntitlementsFeatureId) -> Self {
        Self { id, inner: RetrieveEntitlementsFeatureBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveEntitlementsFeature<'_> {
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

impl StripeRequest for RetrieveEntitlementsFeature<'_> {
    type Output = stripe_shared::EntitlementsFeature;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/entitlements/features/{id}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateEntitlementsFeatureBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    lookup_key: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    name: &'a str,
}
impl<'a> CreateEntitlementsFeatureBuilder<'a> {
    fn new(lookup_key: &'a str, name: &'a str) -> Self {
        Self { expand: None, lookup_key, metadata: None, name }
    }
}
/// Creates a feature
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateEntitlementsFeature<'a> {
    inner: CreateEntitlementsFeatureBuilder<'a>,
}
impl<'a> CreateEntitlementsFeature<'a> {
    /// Construct a new `CreateEntitlementsFeature`.
    pub fn new(lookup_key: &'a str, name: &'a str) -> Self {
        Self { inner: CreateEntitlementsFeatureBuilder::new(lookup_key, name) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of key-value pairs that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl CreateEntitlementsFeature<'_> {
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

impl StripeRequest for CreateEntitlementsFeature<'_> {
    type Output = stripe_shared::EntitlementsFeature;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/entitlements/features").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateEntitlementsFeatureBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}
impl<'a> UpdateEntitlementsFeatureBuilder<'a> {
    fn new() -> Self {
        Self { active: None, expand: None, metadata: None, name: None }
    }
}
/// Update a feature’s metadata or permanently deactivate it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateEntitlementsFeature<'a> {
    inner: UpdateEntitlementsFeatureBuilder<'a>,
    id: &'a stripe_shared::EntitlementsFeatureId,
}
impl<'a> UpdateEntitlementsFeature<'a> {
    /// Construct a new `UpdateEntitlementsFeature`.
    pub fn new(id: &'a stripe_shared::EntitlementsFeatureId) -> Self {
        Self { id, inner: UpdateEntitlementsFeatureBuilder::new() }
    }
    /// Inactive features cannot be attached to new products and will not be returned from the features list endpoint.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of key-value pairs that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The feature's name, for your own purpose, not meant to be displayable to the customer.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
}
impl UpdateEntitlementsFeature<'_> {
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

impl StripeRequest for UpdateEntitlementsFeature<'_> {
    type Output = stripe_shared::EntitlementsFeature;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/entitlements/features/{id}"))
            .form(&self.inner)
    }
}
