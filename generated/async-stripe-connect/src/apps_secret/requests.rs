use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListAppsSecretBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    scope: ListAppsSecretScope<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListAppsSecretBuilder<'a> {
    fn new(scope: ListAppsSecretScope<'a>) -> Self {
        Self { ending_before: None, expand: None, limit: None, scope, starting_after: None }
    }
}
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListAppsSecretScope<'a> {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: ListAppsSecretScopeType,
    /// The user ID.
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}
impl<'a> ListAppsSecretScope<'a> {
    pub fn new(type_: ListAppsSecretScopeType) -> Self {
        Self { type_, user: None }
    }
}
/// The secret scope type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListAppsSecretScopeType {
    Account,
    User,
}
impl ListAppsSecretScopeType {
    pub fn as_str(self) -> &'static str {
        use ListAppsSecretScopeType::*;
        match self {
            Account => "account",
            User => "user",
        }
    }
}

impl std::str::FromStr for ListAppsSecretScopeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ListAppsSecretScopeType"))
    }
}
/// List all secrets stored on the given scope.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListAppsSecret<'a> {
    inner: ListAppsSecretBuilder<'a>,
}
impl<'a> ListAppsSecret<'a> {
    /// Construct a new `ListAppsSecret`.
    pub fn new(scope: ListAppsSecretScope<'a>) -> Self {
        Self { inner: ListAppsSecretBuilder::new(scope) }
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
impl ListAppsSecret<'_> {
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
        stripe_client_core::ListPaginator::new_list("/apps/secrets", self.inner)
    }
}

impl StripeRequest for ListAppsSecret<'_> {
    type Output = stripe_types::List<stripe_connect::AppsSecret>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/apps/secrets").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct FindAppsSecretBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    name: &'a str,
    scope: FindAppsSecretScope<'a>,
}
impl<'a> FindAppsSecretBuilder<'a> {
    fn new(name: &'a str, scope: FindAppsSecretScope<'a>) -> Self {
        Self { expand: None, name, scope }
    }
}
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct FindAppsSecretScope<'a> {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: FindAppsSecretScopeType,
    /// The user ID.
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}
impl<'a> FindAppsSecretScope<'a> {
    pub fn new(type_: FindAppsSecretScopeType) -> Self {
        Self { type_, user: None }
    }
}
/// The secret scope type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FindAppsSecretScopeType {
    Account,
    User,
}
impl FindAppsSecretScopeType {
    pub fn as_str(self) -> &'static str {
        use FindAppsSecretScopeType::*;
        match self {
            Account => "account",
            User => "user",
        }
    }
}

impl std::str::FromStr for FindAppsSecretScopeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FindAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FindAppsSecretScopeType"))
    }
}
/// Finds a secret in the secret store by name and scope.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FindAppsSecret<'a> {
    inner: FindAppsSecretBuilder<'a>,
}
impl<'a> FindAppsSecret<'a> {
    /// Construct a new `FindAppsSecret`.
    pub fn new(name: &'a str, scope: FindAppsSecretScope<'a>) -> Self {
        Self { inner: FindAppsSecretBuilder::new(name, scope) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl FindAppsSecret<'_> {
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

impl StripeRequest for FindAppsSecret<'_> {
    type Output = stripe_connect::AppsSecret;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/apps/secrets/find").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateAppsSecretBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    name: &'a str,
    payload: &'a str,
    scope: CreateAppsSecretScope<'a>,
}
impl<'a> CreateAppsSecretBuilder<'a> {
    fn new(name: &'a str, payload: &'a str, scope: CreateAppsSecretScope<'a>) -> Self {
        Self { expand: None, expires_at: None, name, payload, scope }
    }
}
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAppsSecretScope<'a> {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: CreateAppsSecretScopeType,
    /// The user ID.
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}
impl<'a> CreateAppsSecretScope<'a> {
    pub fn new(type_: CreateAppsSecretScopeType) -> Self {
        Self { type_, user: None }
    }
}
/// The secret scope type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAppsSecretScopeType {
    Account,
    User,
}
impl CreateAppsSecretScopeType {
    pub fn as_str(self) -> &'static str {
        use CreateAppsSecretScopeType::*;
        match self {
            Account => "account",
            User => "user",
        }
    }
}

impl std::str::FromStr for CreateAppsSecretScopeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateAppsSecretScopeType"))
    }
}
/// Create or replace a secret in the secret store.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAppsSecret<'a> {
    inner: CreateAppsSecretBuilder<'a>,
}
impl<'a> CreateAppsSecret<'a> {
    /// Construct a new `CreateAppsSecret`.
    pub fn new(name: &'a str, payload: &'a str, scope: CreateAppsSecretScope<'a>) -> Self {
        Self { inner: CreateAppsSecretBuilder::new(name, payload, scope) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    pub fn expires_at(mut self, expires_at: stripe_types::Timestamp) -> Self {
        self.inner.expires_at = Some(expires_at);
        self
    }
}
impl CreateAppsSecret<'_> {
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

impl StripeRequest for CreateAppsSecret<'_> {
    type Output = stripe_connect::AppsSecret;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/apps/secrets").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct DeleteWhereAppsSecretBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    name: &'a str,
    scope: DeleteWhereAppsSecretScope<'a>,
}
impl<'a> DeleteWhereAppsSecretBuilder<'a> {
    fn new(name: &'a str, scope: DeleteWhereAppsSecretScope<'a>) -> Self {
        Self { expand: None, name, scope }
    }
}
/// Specifies the scoping of the secret.
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DeleteWhereAppsSecretScope<'a> {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: DeleteWhereAppsSecretScopeType,
    /// The user ID.
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}
impl<'a> DeleteWhereAppsSecretScope<'a> {
    pub fn new(type_: DeleteWhereAppsSecretScopeType) -> Self {
        Self { type_, user: None }
    }
}
/// The secret scope type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DeleteWhereAppsSecretScopeType {
    Account,
    User,
}
impl DeleteWhereAppsSecretScopeType {
    pub fn as_str(self) -> &'static str {
        use DeleteWhereAppsSecretScopeType::*;
        match self {
            Account => "account",
            User => "user",
        }
    }
}

impl std::str::FromStr for DeleteWhereAppsSecretScopeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeleteWhereAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for DeleteWhereAppsSecretScopeType")
        })
    }
}
/// Deletes a secret from the secret store by name and scope.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteWhereAppsSecret<'a> {
    inner: DeleteWhereAppsSecretBuilder<'a>,
}
impl<'a> DeleteWhereAppsSecret<'a> {
    /// Construct a new `DeleteWhereAppsSecret`.
    pub fn new(name: &'a str, scope: DeleteWhereAppsSecretScope<'a>) -> Self {
        Self { inner: DeleteWhereAppsSecretBuilder::new(name, scope) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl DeleteWhereAppsSecret<'_> {
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

impl StripeRequest for DeleteWhereAppsSecret<'_> {
    type Output = stripe_connect::AppsSecret;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/apps/secrets/delete").form(&self.inner)
    }
}
