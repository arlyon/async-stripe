#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListAppsSecret<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Specifies the scoping of the secret.
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: ListAppsSecretScope<'a>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListAppsSecret<'a> {
    pub fn new(scope: ListAppsSecretScope<'a>) -> Self {
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(()),
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
impl<'a> ListAppsSecret<'a> {
    /// List all secrets stored on the given scope.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_connect::AppsSecret>> {
        client.get_query("/apps/secrets", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::List<stripe_connect::AppsSecret>> {
        stripe::ListPaginator::from_list_params("/apps/secrets", self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct FindAppsSecret<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A name for the secret that's unique within the scope.
    pub name: &'a str,
    /// Specifies the scoping of the secret.
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: FindAppsSecretScope<'a>,
}
impl<'a> FindAppsSecret<'a> {
    pub fn new(name: &'a str, scope: FindAppsSecretScope<'a>) -> Self {
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FindAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(()),
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
impl<'a> FindAppsSecret<'a> {
    /// Finds a secret in the secret store by name and scope.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_connect::AppsSecret> {
        client.get_query("/apps/secrets/find", self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAppsSecret<'a> {
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
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: CreateAppsSecretScope<'a>,
}
impl<'a> CreateAppsSecret<'a> {
    pub fn new(name: &'a str, payload: &'a str, scope: CreateAppsSecretScope<'a>) -> Self {
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(()),
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
impl<'a> CreateAppsSecret<'a> {
    /// Create or replace a secret in the secret store.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_connect::AppsSecret> {
        client.send_form("/apps/secrets", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DeleteWhereAppsSecret<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A name for the secret that's unique within the scope.
    pub name: &'a str,
    /// Specifies the scoping of the secret.
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: DeleteWhereAppsSecretScope<'a>,
}
impl<'a> DeleteWhereAppsSecret<'a> {
    pub fn new(name: &'a str, scope: DeleteWhereAppsSecretScope<'a>) -> Self {
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeleteWhereAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(()),
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
impl<'a> DeleteWhereAppsSecret<'a> {
    /// Deletes a secret from the secret store by name and scope.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_connect::AppsSecret> {
        client.send_form("/apps/secrets/delete", self, http_types::Method::Post)
    }
}
