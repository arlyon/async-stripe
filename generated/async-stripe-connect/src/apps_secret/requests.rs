use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListAppsSecretBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    scope: ScopeParam,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListAppsSecretBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListAppsSecretBuilder").finish_non_exhaustive()
    }
}
impl ListAppsSecretBuilder {
    fn new(scope: impl Into<ScopeParam>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            limit: None,
            scope: scope.into(),
            starting_after: None,
        }
    }
}
/// List all secrets stored on the given scope.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListAppsSecret {
    inner: ListAppsSecretBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListAppsSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListAppsSecret").finish_non_exhaustive()
    }
}
impl ListAppsSecret {
    /// Construct a new `ListAppsSecret`.
    pub fn new(scope: impl Into<ScopeParam>) -> Self {
        Self { inner: ListAppsSecretBuilder::new(scope.into()) }
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
}
impl ListAppsSecret {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_connect::AppsSecret>> {
        stripe_client_core::ListPaginator::new_list("/apps/secrets", &self.inner)
    }
}

impl StripeRequest for ListAppsSecret {
    type Output = stripe_types::List<stripe_connect::AppsSecret>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/apps/secrets").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct FindAppsSecretBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    name: String,
    scope: ScopeParam,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FindAppsSecretBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FindAppsSecretBuilder").finish_non_exhaustive()
    }
}
impl FindAppsSecretBuilder {
    fn new(name: impl Into<String>, scope: impl Into<ScopeParam>) -> Self {
        Self { expand: None, name: name.into(), scope: scope.into() }
    }
}
/// Finds a secret in the secret store by name and scope.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FindAppsSecret {
    inner: FindAppsSecretBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FindAppsSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FindAppsSecret").finish_non_exhaustive()
    }
}
impl FindAppsSecret {
    /// Construct a new `FindAppsSecret`.
    pub fn new(name: impl Into<String>, scope: impl Into<ScopeParam>) -> Self {
        Self { inner: FindAppsSecretBuilder::new(name.into(), scope.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl FindAppsSecret {
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

impl StripeRequest for FindAppsSecret {
    type Output = stripe_connect::AppsSecret;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/apps/secrets/find").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateAppsSecretBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    name: String,
    payload: String,
    scope: ScopeParam,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAppsSecretBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAppsSecretBuilder").finish_non_exhaustive()
    }
}
impl CreateAppsSecretBuilder {
    fn new(
        name: impl Into<String>,
        payload: impl Into<String>,
        scope: impl Into<ScopeParam>,
    ) -> Self {
        Self {
            expand: None,
            expires_at: None,
            name: name.into(),
            payload: payload.into(),
            scope: scope.into(),
        }
    }
}
/// Create or replace a secret in the secret store.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAppsSecret {
    inner: CreateAppsSecretBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAppsSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAppsSecret").finish_non_exhaustive()
    }
}
impl CreateAppsSecret {
    /// Construct a new `CreateAppsSecret`.
    pub fn new(
        name: impl Into<String>,
        payload: impl Into<String>,
        scope: impl Into<ScopeParam>,
    ) -> Self {
        Self { inner: CreateAppsSecretBuilder::new(name.into(), payload.into(), scope.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
}
impl CreateAppsSecret {
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

impl StripeRequest for CreateAppsSecret {
    type Output = stripe_connect::AppsSecret;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/apps/secrets").form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct DeleteWhereAppsSecretBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    name: String,
    scope: ScopeParam,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeleteWhereAppsSecretBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeleteWhereAppsSecretBuilder").finish_non_exhaustive()
    }
}
impl DeleteWhereAppsSecretBuilder {
    fn new(name: impl Into<String>, scope: impl Into<ScopeParam>) -> Self {
        Self { expand: None, name: name.into(), scope: scope.into() }
    }
}
/// Deletes a secret from the secret store by name and scope.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeleteWhereAppsSecret {
    inner: DeleteWhereAppsSecretBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeleteWhereAppsSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeleteWhereAppsSecret").finish_non_exhaustive()
    }
}
impl DeleteWhereAppsSecret {
    /// Construct a new `DeleteWhereAppsSecret`.
    pub fn new(name: impl Into<String>, scope: impl Into<ScopeParam>) -> Self {
        Self { inner: DeleteWhereAppsSecretBuilder::new(name.into(), scope.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DeleteWhereAppsSecret {
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

impl StripeRequest for DeleteWhereAppsSecret {
    type Output = stripe_connect::AppsSecret;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/apps/secrets/delete").form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ScopeParam {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: stripe_shared::FindAppsSecretScopeType,
    /// The user ID.
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ScopeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ScopeParam").finish_non_exhaustive()
    }
}
impl ScopeParam {
    pub fn new(type_: impl Into<stripe_shared::FindAppsSecretScopeType>) -> Self {
        Self { type_: type_.into(), user: None }
    }
}
/// The secret scope type.
pub type ListAppsSecretScopeType = stripe_shared::FindAppsSecretScopeType;
/// The secret scope type.
pub type FindAppsSecretScopeType = stripe_shared::FindAppsSecretScopeType;
/// The secret scope type.
pub type CreateAppsSecretScopeType = stripe_shared::FindAppsSecretScopeType;
/// The secret scope type.
pub type DeleteWhereAppsSecretScopeType = stripe_shared::FindAppsSecretScopeType;
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
pub type ListAppsSecretScope = ScopeParam;
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
pub type FindAppsSecretScope = ScopeParam;
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
pub type CreateAppsSecretScope = ScopeParam;
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
pub type DeleteWhereAppsSecretScope = ScopeParam;
