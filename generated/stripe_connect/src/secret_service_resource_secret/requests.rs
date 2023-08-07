#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct FindSecretServiceResourceSecret<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A name for the secret that's unique within the scope.
    pub name: &'a str,
    /// Specifies the scoping of the secret.
    ///
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: ScopeParam<'a>,
}
impl<'a> FindSecretServiceResourceSecret<'a> {
    pub fn new(name: &'a str, scope: ScopeParam<'a>) -> Self {
        Self { expand: Default::default(), name, scope }
    }
}
impl<'a> FindSecretServiceResourceSecret<'a> {
    /// Finds a secret in the secret store by name and scope.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_connect::SecretServiceResourceSecret> {
        client.get_query("/apps/secrets/find", self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSecretServiceResourceSecret<'a> {
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
    pub scope: ScopeParam<'a>,
}
impl<'a> CreateSecretServiceResourceSecret<'a> {
    pub fn new(name: &'a str, payload: &'a str, scope: ScopeParam<'a>) -> Self {
        Self { expand: Default::default(), expires_at: Default::default(), name, payload, scope }
    }
}
impl<'a> CreateSecretServiceResourceSecret<'a> {
    /// Create or replace a secret in the secret store.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_connect::SecretServiceResourceSecret> {
        client.send_form("/apps/secrets", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DeleteWhereSecretServiceResourceSecret<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A name for the secret that's unique within the scope.
    pub name: &'a str,
    /// Specifies the scoping of the secret.
    ///
    /// Requests originating from UI extensions can only access account-scoped secrets or secrets scoped to their own user.
    pub scope: ScopeParam<'a>,
}
impl<'a> DeleteWhereSecretServiceResourceSecret<'a> {
    pub fn new(name: &'a str, scope: ScopeParam<'a>) -> Self {
        Self { expand: Default::default(), name, scope }
    }
}
impl<'a> DeleteWhereSecretServiceResourceSecret<'a> {
    /// Deletes a secret from the secret store by name and scope.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_connect::SecretServiceResourceSecret> {
        client.send_form("/apps/secrets/delete", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListSecretServiceResourceSecret<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub scope: ScopeParam<'a>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListSecretServiceResourceSecret<'a> {
    pub fn new(scope: ScopeParam<'a>) -> Self {
        Self {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            scope,
            starting_after: Default::default(),
        }
    }
}
impl<'a> stripe::PaginationParams for ListSecretServiceResourceSecret<'a> {}
impl<'a> ListSecretServiceResourceSecret<'a> {
    /// List all secrets stored on the given scope.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_connect::SecretServiceResourceSecret>> {
        client.get_query("/apps/secrets", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_connect::SecretServiceResourceSecret> {
        stripe::ListPaginator::from_params("/apps/secrets", self)
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Type {
    Account,
    User,
}

impl Type {
    pub fn as_str(self) -> &'static str {
        use Type::*;
        match self {
            Account => "account",
            User => "user",
        }
    }
}

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Type::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ScopeParam<'a> {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: Type,
    /// The user ID.
    ///
    /// This field is required if `type` is set to `user`, and should not be provided if `type` is set to `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}
impl<'a> ScopeParam<'a> {
    pub fn new(type_: Type) -> Self {
        Self { type_, user: Default::default() }
    }
}
