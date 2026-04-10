use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListEntitlementsFeatureBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListEntitlementsFeatureBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListEntitlementsFeatureBuilder").finish_non_exhaustive()
    }
}
impl ListEntitlementsFeatureBuilder {
    fn new() -> Self {
        Self {
            archived: None,
            ending_before: None,
            expand: None,
            limit: None,
            lookup_key: None,
            starting_after: None,
        }
    }
}
/// Retrieve a list of features
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListEntitlementsFeature {
    inner: ListEntitlementsFeatureBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListEntitlementsFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListEntitlementsFeature").finish_non_exhaustive()
    }
}
impl ListEntitlementsFeature {
    /// Construct a new `ListEntitlementsFeature`.
    pub fn new() -> Self {
        Self { inner: ListEntitlementsFeatureBuilder::new() }
    }
    /// If set, filter results to only include features with the given archive status.
    pub fn archived(mut self, archived: impl Into<bool>) -> Self {
        self.inner.archived = Some(archived.into());
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
    /// If set, filter results to only include features with the given lookup_key.
    pub fn lookup_key(mut self, lookup_key: impl Into<String>) -> Self {
        self.inner.lookup_key = Some(lookup_key.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListEntitlementsFeature {
    fn default() -> Self {
        Self::new()
    }
}
impl ListEntitlementsFeature {
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
        stripe_client_core::ListPaginator::new_list("/entitlements/features", &self.inner)
    }
}

impl StripeRequest for ListEntitlementsFeature {
    type Output = stripe_types::List<stripe_shared::EntitlementsFeature>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/entitlements/features").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveEntitlementsFeatureBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveEntitlementsFeatureBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveEntitlementsFeatureBuilder").finish_non_exhaustive()
    }
}
impl RetrieveEntitlementsFeatureBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a feature
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveEntitlementsFeature {
    inner: RetrieveEntitlementsFeatureBuilder,
    id: stripe_shared::EntitlementsFeatureId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveEntitlementsFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveEntitlementsFeature").finish_non_exhaustive()
    }
}
impl RetrieveEntitlementsFeature {
    /// Construct a new `RetrieveEntitlementsFeature`.
    pub fn new(id: impl Into<stripe_shared::EntitlementsFeatureId>) -> Self {
        Self { id: id.into(), inner: RetrieveEntitlementsFeatureBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveEntitlementsFeature {
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

impl StripeRequest for RetrieveEntitlementsFeature {
    type Output = stripe_shared::EntitlementsFeature;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/entitlements/features/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateEntitlementsFeatureBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    lookup_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    name: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateEntitlementsFeatureBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateEntitlementsFeatureBuilder").finish_non_exhaustive()
    }
}
impl CreateEntitlementsFeatureBuilder {
    fn new(lookup_key: impl Into<String>, name: impl Into<String>) -> Self {
        Self { expand: None, lookup_key: lookup_key.into(), metadata: None, name: name.into() }
    }
}
/// Creates a feature
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateEntitlementsFeature {
    inner: CreateEntitlementsFeatureBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateEntitlementsFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateEntitlementsFeature").finish_non_exhaustive()
    }
}
impl CreateEntitlementsFeature {
    /// Construct a new `CreateEntitlementsFeature`.
    pub fn new(lookup_key: impl Into<String>, name: impl Into<String>) -> Self {
        Self { inner: CreateEntitlementsFeatureBuilder::new(lookup_key.into(), name.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of key-value pairs that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl CreateEntitlementsFeature {
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

impl StripeRequest for CreateEntitlementsFeature {
    type Output = stripe_shared::EntitlementsFeature;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/entitlements/features").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateEntitlementsFeatureBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateEntitlementsFeatureBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateEntitlementsFeatureBuilder").finish_non_exhaustive()
    }
}
impl UpdateEntitlementsFeatureBuilder {
    fn new() -> Self {
        Self { active: None, expand: None, metadata: None, name: None }
    }
}
/// Update a feature’s metadata or permanently deactivate it.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateEntitlementsFeature {
    inner: UpdateEntitlementsFeatureBuilder,
    id: stripe_shared::EntitlementsFeatureId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateEntitlementsFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateEntitlementsFeature").finish_non_exhaustive()
    }
}
impl UpdateEntitlementsFeature {
    /// Construct a new `UpdateEntitlementsFeature`.
    pub fn new(id: impl Into<stripe_shared::EntitlementsFeatureId>) -> Self {
        Self { id: id.into(), inner: UpdateEntitlementsFeatureBuilder::new() }
    }
    /// Inactive features cannot be attached to new products and will not be returned from the features list endpoint.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of key-value pairs that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The feature's name, for your own purpose, not meant to be displayable to the customer.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
}
impl UpdateEntitlementsFeature {
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

impl StripeRequest for UpdateEntitlementsFeature {
    type Output = stripe_shared::EntitlementsFeature;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/entitlements/features/{id}"))
            .form(&self.inner)
    }
}
