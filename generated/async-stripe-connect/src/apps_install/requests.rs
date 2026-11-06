use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListAppsInstallBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<ListAppsInstallChannel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_connect::AppsInstallStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListAppsInstallBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListAppsInstallBuilder").finish_non_exhaustive()
    }
}
impl ListAppsInstallBuilder {
    fn new() -> Self {
        Self {
            account: None,
            app: None,
            approval_required: None,
            channel: None,
            created: None,
            created_by: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Only return installs in the distribution channel specified by this channel name.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListAppsInstallChannel {
    PrivateLive,
    PrivateTest,
    Public,
    Testing,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListAppsInstallChannel {
    pub fn as_str(&self) -> &str {
        use ListAppsInstallChannel::*;
        match self {
            PrivateLive => "private_live",
            PrivateTest => "private_test",
            Public => "public",
            Testing => "testing",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListAppsInstallChannel {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListAppsInstallChannel::*;
        match s {
            "private_live" => Ok(PrivateLive),
            "private_test" => Ok(PrivateTest),
            "public" => Ok(Public),
            "testing" => Ok(Testing),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ListAppsInstallChannel");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ListAppsInstallChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ListAppsInstallChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListAppsInstallChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ListAppsInstallChannel)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ListAppsInstallChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListAppsInstallChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Returns a list of app installs.
/// An app developer filtering by its own app with its own key sees that app’s installs across the accounts that installed it.
/// An app developer acting on a connected account through `Stripe-Account` and filtering by its app sees that account’s installs of the app, and an embedding platform acting on a connected account sees only the installs it created there.
/// Other callers see the installs on their own account.
/// A live key lists live installs and a test key lists test installs; the key of an app’s managed sandbox filtering by `app` lists that app’s installs across every sandbox.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListAppsInstall {
    inner: ListAppsInstallBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListAppsInstall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListAppsInstall").finish_non_exhaustive()
    }
}
impl ListAppsInstall {
    /// Construct a new `ListAppsInstall`.
    pub fn new() -> Self {
        Self { inner: ListAppsInstallBuilder::new() }
    }
    /// Only return installs made by this account.
    /// Only useful to an app developer filtering by its own app with its own key, whose list spans the accounts that installed the app; other lists only accept the authenticated account.
    pub fn account(mut self, account: impl Into<String>) -> Self {
        self.inner.account = Some(account.into());
        self
    }
    /// Only return installs for the app specified by this app ID.
    pub fn app(mut self, app: impl Into<String>) -> Self {
        self.inner.app = Some(app.into());
        self
    }
    /// Only return installs whose installer must authorize pending permissions, content security policy entries, or endpoints.
    pub fn approval_required(mut self, approval_required: impl Into<bool>) -> Self {
        self.inner.approval_required = Some(approval_required.into());
        self
    }
    /// Only return installs in the distribution channel specified by this channel name.
    pub fn channel(mut self, channel: impl Into<ListAppsInstallChannel>) -> Self {
        self.inner.channel = Some(channel.into());
        self
    }
    /// Only return app installs that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return installs created by the embedding platform specified by this account ID.
    pub fn created_by(mut self, created_by: impl Into<String>) -> Self {
        self.inner.created_by = Some(created_by.into());
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return installs with the given status: `installed`, `installing`, `install_failed`, `uninstalling`, or `uninstall_failed`.
    pub fn status(mut self, status: impl Into<stripe_connect::AppsInstallStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListAppsInstall {
    fn default() -> Self {
        Self::new()
    }
}
impl ListAppsInstall {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_connect::AppsInstall>> {
        stripe_client_core::ListPaginator::new_list("/apps/installs", &self.inner)
    }
}

impl StripeRequest for ListAppsInstall {
    type Output = stripe_types::List<stripe_connect::AppsInstall>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/apps/installs").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveAppsInstallBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveAppsInstallBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveAppsInstallBuilder").finish_non_exhaustive()
    }
}
impl RetrieveAppsInstallBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an app install.
/// The installing account, the app’s developer (with the keys of the account that owns the app or of the app’s managed sandbox), and the embedding platform that created the install can retrieve it.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveAppsInstall {
    inner: RetrieveAppsInstallBuilder,
    id: stripe_connect::AppsInstallId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveAppsInstall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveAppsInstall").finish_non_exhaustive()
    }
}
impl RetrieveAppsInstall {
    /// Construct a new `RetrieveAppsInstall`.
    pub fn new(id: impl Into<stripe_connect::AppsInstallId>) -> Self {
        Self { id: id.into(), inner: RetrieveAppsInstallBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveAppsInstall {
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

impl StripeRequest for RetrieveAppsInstall {
    type Output = stripe_connect::AppsInstall;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/apps/installs/{id}")).query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateAppsInstallBuilder {
    app: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<CreateAppsInstallChannel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_challenge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_challenge_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAppsInstallBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAppsInstallBuilder").finish_non_exhaustive()
    }
}
impl CreateAppsInstallBuilder {
    fn new(app: impl Into<String>) -> Self {
        Self {
            app: app.into(),
            channel: None,
            code_challenge: None,
            code_challenge_method: None,
            expand: None,
        }
    }
}
/// The distribution channel to install from.
/// Defaults to `public`, which only app developers and embedding platforms can use.
/// An account installing its own private app must pass `private_test` or `private_live`, matching the mode of the API key.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAppsInstallChannel {
    PrivateLive,
    PrivateTest,
    Public,
    Testing,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAppsInstallChannel {
    pub fn as_str(&self) -> &str {
        use CreateAppsInstallChannel::*;
        match self {
            PrivateLive => "private_live",
            PrivateTest => "private_test",
            Public => "public",
            Testing => "testing",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAppsInstallChannel {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAppsInstallChannel::*;
        match s {
            "private_live" => Ok(PrivateLive),
            "private_test" => Ok(PrivateTest),
            "public" => Ok(Public),
            "testing" => Ok(Testing),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateAppsInstallChannel");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAppsInstallChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAppsInstallChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAppsInstallChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAppsInstallChannel)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAppsInstallChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAppsInstallChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Creates an app install.
/// An account installs its own private app with its own key; public and testing installs are made from the Dashboard.
/// An app developer acting on a connected account through `Stripe-Account` installs or reinstalls its app there, and an embedding platform can do the same once the app’s developer approves its request to embed the app.
/// For a private app, creating an install installs the newest completed upload; when that version is already installed with nothing pending, the existing install is returned.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAppsInstall {
    inner: CreateAppsInstallBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAppsInstall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAppsInstall").finish_non_exhaustive()
    }
}
impl CreateAppsInstall {
    /// Construct a new `CreateAppsInstall`.
    pub fn new(app: impl Into<String>) -> Self {
        Self { inner: CreateAppsInstallBuilder::new(app.into()) }
    }
    /// The distribution channel to install from.
    /// Defaults to `public`, which only app developers and embedding platforms can use.
    /// An account installing its own private app must pass `private_test` or `private_live`, matching the mode of the API key.
    pub fn channel(mut self, channel: impl Into<CreateAppsInstallChannel>) -> Self {
        self.inner.channel = Some(channel.into());
        self
    }
    /// For OAuth apps, the PKCE code challenge used to issue the `auth_code` returned on the install.
    /// Must be 43 to 128 characters and contain only letters, numbers, `-`, `.`, `_`, and `~`.
    /// Only applies to installs made by the app developer or an embedding platform; ignored when an account installs its own private app.
    pub fn code_challenge(mut self, code_challenge: impl Into<String>) -> Self {
        self.inner.code_challenge = Some(code_challenge.into());
        self
    }
    /// The method used to derive `code_challenge`.
    /// Required when `code_challenge` is provided, and must be `S256`.
    pub fn code_challenge_method(mut self, code_challenge_method: impl Into<String>) -> Self {
        self.inner.code_challenge_method = Some(code_challenge_method.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateAppsInstall {
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

impl StripeRequest for CreateAppsInstall {
    type Output = stripe_connect::AppsInstall;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/apps/installs").form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateAppsInstallBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAppsInstallBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAppsInstallBuilder").finish_non_exhaustive()
    }
}
impl UpdateAppsInstallBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Reauthorizes an app install.
/// The installer grants the permissions, content security policy entries, and endpoints that the version being installed requests.
/// An account reauthorizes its own installs on any channel with its own key, which grants all of that access, so only give `app_install_write` to keys that may approve an app’s access.
/// App developers and embedding platforms reauthorize installs on connected accounts through `Stripe-Account`.
/// An app developer can’t grant new access.
/// An embedding platform can grant new access only once the app’s developer approves its request to embed the app.
/// For private apps, the version being installed is the newest completed upload.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAppsInstall {
    inner: UpdateAppsInstallBuilder,
    id: stripe_connect::AppsInstallId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAppsInstall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAppsInstall").finish_non_exhaustive()
    }
}
impl UpdateAppsInstall {
    /// Construct a new `UpdateAppsInstall`.
    pub fn new(id: impl Into<stripe_connect::AppsInstallId>) -> Self {
        Self { id: id.into(), inner: UpdateAppsInstallBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl UpdateAppsInstall {
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

impl StripeRequest for UpdateAppsInstall {
    type Output = stripe_connect::AppsInstall;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/apps/installs/{id}")).form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UninstallAppsInstallBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UninstallAppsInstallBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UninstallAppsInstallBuilder").finish_non_exhaustive()
    }
}
impl UninstallAppsInstallBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Uninstalls an app from the account that installed it.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UninstallAppsInstall {
    inner: UninstallAppsInstallBuilder,
    id: stripe_connect::AppsInstallId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UninstallAppsInstall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UninstallAppsInstall").finish_non_exhaustive()
    }
}
impl UninstallAppsInstall {
    /// Construct a new `UninstallAppsInstall`.
    pub fn new(id: impl Into<stripe_connect::AppsInstallId>) -> Self {
        Self { id: id.into(), inner: UninstallAppsInstallBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl UninstallAppsInstall {
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

impl StripeRequest for UninstallAppsInstall {
    type Output = stripe_connect::AppsInstall;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/apps/installs/{id}/uninstall"))
            .form(&self.inner)
    }
}
