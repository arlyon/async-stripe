use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListAppsSecretBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    scope: ListAppsSecretScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListAppsSecretBuilder {
    fn new(scope: impl Into<ListAppsSecretScope>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            limit: None,
            scope: scope.into(),
            starting_after: None,
        }
    }
}
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ListAppsSecretScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: ListAppsSecretScopeType,
    /// The user ID.
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl ListAppsSecretScope {
    pub fn new(type_: impl Into<ListAppsSecretScopeType>) -> Self {
        Self { type_: type_.into(), user: None }
    }
}
/// The secret scope type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListAppsSecretScopeType {
    Account,
    User,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListAppsSecretScopeType {
    pub fn as_str(&self) -> &str {
        use ListAppsSecretScopeType::*;
        match self {
            Account => "account",
            User => "user",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListAppsSecretScopeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ListAppsSecretScopeType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ListAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListAppsSecretScopeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListAppsSecretScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// List all secrets stored on the given scope.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListAppsSecret {
    inner: ListAppsSecretBuilder,
}
impl ListAppsSecret {
    /// Construct a new `ListAppsSecret`.
    pub fn new(scope: impl Into<ListAppsSecretScope>) -> Self {
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct FindAppsSecretBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    name: String,
    scope: FindAppsSecretScope,
}
impl FindAppsSecretBuilder {
    fn new(name: impl Into<String>, scope: impl Into<FindAppsSecretScope>) -> Self {
        Self { expand: None, name: name.into(), scope: scope.into() }
    }
}
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct FindAppsSecretScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: FindAppsSecretScopeType,
    /// The user ID.
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl FindAppsSecretScope {
    pub fn new(type_: impl Into<FindAppsSecretScopeType>) -> Self {
        Self { type_: type_.into(), user: None }
    }
}
/// The secret scope type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FindAppsSecretScopeType {
    Account,
    User,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FindAppsSecretScopeType {
    pub fn as_str(&self) -> &str {
        use FindAppsSecretScopeType::*;
        match self {
            Account => "account",
            User => "user",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FindAppsSecretScopeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FindAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "FindAppsSecretScopeType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FindAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FindAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FindAppsSecretScopeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FindAppsSecretScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Finds a secret in the secret store by name and scope.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FindAppsSecret {
    inner: FindAppsSecretBuilder,
}
impl FindAppsSecret {
    /// Construct a new `FindAppsSecret`.
    pub fn new(name: impl Into<String>, scope: impl Into<FindAppsSecretScope>) -> Self {
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct CreateAppsSecretBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    name: String,
    payload: String,
    scope: CreateAppsSecretScope,
}
impl CreateAppsSecretBuilder {
    fn new(
        name: impl Into<String>,
        payload: impl Into<String>,
        scope: impl Into<CreateAppsSecretScope>,
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
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateAppsSecretScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: CreateAppsSecretScopeType,
    /// The user ID.
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl CreateAppsSecretScope {
    pub fn new(type_: impl Into<CreateAppsSecretScopeType>) -> Self {
        Self { type_: type_.into(), user: None }
    }
}
/// The secret scope type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAppsSecretScopeType {
    Account,
    User,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAppsSecretScopeType {
    pub fn as_str(&self) -> &str {
        use CreateAppsSecretScopeType::*;
        match self {
            Account => "account",
            User => "user",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAppsSecretScopeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateAppsSecretScopeType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateAppsSecretScopeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAppsSecretScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Create or replace a secret in the secret store.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAppsSecret {
    inner: CreateAppsSecretBuilder,
}
impl CreateAppsSecret {
    /// Construct a new `CreateAppsSecret`.
    pub fn new(
        name: impl Into<String>,
        payload: impl Into<String>,
        scope: impl Into<CreateAppsSecretScope>,
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct DeleteWhereAppsSecretBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    name: String,
    scope: DeleteWhereAppsSecretScope,
}
impl DeleteWhereAppsSecretBuilder {
    fn new(name: impl Into<String>, scope: impl Into<DeleteWhereAppsSecretScope>) -> Self {
        Self { expand: None, name: name.into(), scope: scope.into() }
    }
}
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct DeleteWhereAppsSecretScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: DeleteWhereAppsSecretScopeType,
    /// The user ID.
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl DeleteWhereAppsSecretScope {
    pub fn new(type_: impl Into<DeleteWhereAppsSecretScopeType>) -> Self {
        Self { type_: type_.into(), user: None }
    }
}
/// The secret scope type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DeleteWhereAppsSecretScopeType {
    Account,
    User,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DeleteWhereAppsSecretScopeType {
    pub fn as_str(&self) -> &str {
        use DeleteWhereAppsSecretScopeType::*;
        match self {
            Account => "account",
            User => "user",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DeleteWhereAppsSecretScopeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeleteWhereAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DeleteWhereAppsSecretScopeType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DeleteWhereAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DeleteWhereAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DeleteWhereAppsSecretScopeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DeleteWhereAppsSecretScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Deletes a secret from the secret store by name and scope.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteWhereAppsSecret {
    inner: DeleteWhereAppsSecretBuilder,
}
impl DeleteWhereAppsSecret {
    /// Construct a new `DeleteWhereAppsSecret`.
    pub fn new(name: impl Into<String>, scope: impl Into<DeleteWhereAppsSecretScope>) -> Self {
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
