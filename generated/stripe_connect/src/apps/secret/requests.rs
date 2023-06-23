use stripe::{Client, Response};

impl stripe_connect::apps::secret::Secret {
    /// Finds a secret in the secret store by name and scope.
    pub fn find(
        client: &Client,
        params: FindSecret,
    ) -> Response<stripe_connect::apps::secret::Secret> {
        client.get_query("/apps/secrets/find", params)
    }
    /// Create or replace a secret in the secret store.
    pub fn create(
        client: &Client,
        params: CreateSecret,
    ) -> Response<stripe_connect::apps::secret::Secret> {
        client.send_form("/apps/secrets", params, http_types::Method::Post)
    }
    /// Deletes a secret from the secret store by name and scope.
    pub fn delete_where(
        client: &Client,
        params: DeleteWhereSecret,
    ) -> Response<stripe_connect::apps::secret::Secret> {
        client.send_form("/apps/secrets/delete", params, http_types::Method::Post)
    }
    /// List all secrets stored on the given scope.
    pub fn list(
        client: &Client,
        params: ListSecret,
    ) -> Response<stripe_types::List<stripe_connect::apps::secret::Secret>> {
        client.get_query("/apps/secrets", params)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct FindSecret<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A name for the secret that's unique within the scope.
    pub name: &'a str,
    /// Specifies the scoping of the secret.
    ///
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: FindSecretScope<'a>,
}
impl<'a> FindSecret<'a> {
    pub fn new(name: &'a str, scope: FindSecretScope<'a>) -> Self {
        Self { expand: Default::default(), name, scope }
    }
}
/// Specifies the scoping of the secret.
///
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct FindSecretScope<'a> {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: FindSecretScopeType,
    /// The user ID.
    ///
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}
impl<'a> FindSecretScope<'a> {
    pub fn new(type_: FindSecretScopeType) -> Self {
        Self { type_, user: Default::default() }
    }
}
/// The secret scope type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FindSecretScopeType {
    Account,
    User,
}

impl FindSecretScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for FindSecretScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FindSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSecret<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// A name for the secret that's unique within the scope.
    pub name: &'a str,
    /// The plaintext secret value to be stored.
    pub payload: &'a str,
    /// Specifies the scoping of the secret.
    ///
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: CreateSecretScope<'a>,
}
impl<'a> CreateSecret<'a> {
    pub fn new(name: &'a str, payload: &'a str, scope: CreateSecretScope<'a>) -> Self {
        Self { expand: Default::default(), expires_at: Default::default(), name, payload, scope }
    }
}
/// Specifies the scoping of the secret.
///
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSecretScope<'a> {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: CreateSecretScopeType,
    /// The user ID.
    ///
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}
impl<'a> CreateSecretScope<'a> {
    pub fn new(type_: CreateSecretScopeType) -> Self {
        Self { type_, user: Default::default() }
    }
}
/// The secret scope type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSecretScopeType {
    Account,
    User,
}

impl CreateSecretScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for CreateSecretScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DeleteWhereSecret<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A name for the secret that's unique within the scope.
    pub name: &'a str,
    /// Specifies the scoping of the secret.
    ///
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: DeleteWhereSecretScope<'a>,
}
impl<'a> DeleteWhereSecret<'a> {
    pub fn new(name: &'a str, scope: DeleteWhereSecretScope<'a>) -> Self {
        Self { expand: Default::default(), name, scope }
    }
}
/// Specifies the scoping of the secret.
///
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DeleteWhereSecretScope<'a> {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: DeleteWhereSecretScopeType,
    /// The user ID.
    ///
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}
impl<'a> DeleteWhereSecretScope<'a> {
    pub fn new(type_: DeleteWhereSecretScopeType) -> Self {
        Self { type_, user: Default::default() }
    }
}
/// The secret scope type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteWhereSecretScopeType {
    Account,
    User,
}

impl DeleteWhereSecretScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for DeleteWhereSecretScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeleteWhereSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListSecret<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Specifies the scoping of the secret.
    ///
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: ListSecretScope<'a>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListSecret<'a> {
    pub fn new(scope: ListSecretScope<'a>) -> Self {
        Self {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            scope,
            starting_after: Default::default(),
        }
    }
}
/// Specifies the scoping of the secret.
///
/// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListSecretScope<'a> {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: ListSecretScopeType,
    /// The user ID.
    ///
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}
impl<'a> ListSecretScope<'a> {
    pub fn new(type_: ListSecretScopeType) -> Self {
        Self { type_, user: Default::default() }
    }
}
/// The secret scope type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListSecretScopeType {
    Account,
    User,
}

impl ListSecretScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for ListSecretScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
