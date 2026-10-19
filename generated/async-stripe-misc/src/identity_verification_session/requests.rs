use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::IdentityVerificationSessionStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIdentityVerificationSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIdentityVerificationSessionBuilder").finish_non_exhaustive()
    }
}
impl ListIdentityVerificationSessionBuilder {
    fn new() -> Self {
        Self {
            client_reference_id: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            related_customer: None,
            related_customer_account: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of VerificationSessions
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListIdentityVerificationSession {
    inner: ListIdentityVerificationSessionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIdentityVerificationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIdentityVerificationSession").finish_non_exhaustive()
    }
}
impl ListIdentityVerificationSession {
    /// Construct a new `ListIdentityVerificationSession`.
    pub fn new() -> Self {
        Self { inner: ListIdentityVerificationSessionBuilder::new() }
    }
    /// A string to reference this user.
    /// This can be a customer ID, a session ID, or similar, and can be used to reconcile this verification with your internal systems.
    pub fn client_reference_id(mut self, client_reference_id: impl Into<String>) -> Self {
        self.inner.client_reference_id = Some(client_reference_id.into());
        self
    }
    /// Only return VerificationSessions that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
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
    /// Customer ID
    pub fn related_customer(mut self, related_customer: impl Into<String>) -> Self {
        self.inner.related_customer = Some(related_customer.into());
        self
    }
    /// The ID of the Account representing a customer.
    pub fn related_customer_account(mut self, related_customer_account: impl Into<String>) -> Self {
        self.inner.related_customer_account = Some(related_customer_account.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return VerificationSessions with this status.
    /// [Learn more about the lifecycle of sessions](https://docs.stripe.com/identity/how-sessions-work).
    pub fn status(
        mut self,
        status: impl Into<stripe_shared::IdentityVerificationSessionStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListIdentityVerificationSession {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIdentityVerificationSession {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_misc::IdentityVerificationSession>,
    > {
        stripe_client_core::ListPaginator::new_list("/identity/verification_sessions", &self.inner)
    }
}

impl StripeRequest for ListIdentityVerificationSession {
    type Output = stripe_types::List<stripe_misc::IdentityVerificationSession>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/identity/verification_sessions").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIdentityVerificationSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIdentityVerificationSessionBuilder").finish_non_exhaustive()
    }
}
impl RetrieveIdentityVerificationSessionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a VerificationSession that was previously created.
///
/// When the session status is `requires_input`, you can use this method to retrieve a valid
/// `client_secret` or `url` to allow re-submission.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveIdentityVerificationSession {
    inner: RetrieveIdentityVerificationSessionBuilder,
    session: stripe_misc::IdentityVerificationSessionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIdentityVerificationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIdentityVerificationSession").finish_non_exhaustive()
    }
}
impl RetrieveIdentityVerificationSession {
    /// Construct a new `RetrieveIdentityVerificationSession`.
    pub fn new(session: impl Into<stripe_misc::IdentityVerificationSessionId>) -> Self {
        Self { session: session.into(), inner: RetrieveIdentityVerificationSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIdentityVerificationSession {
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

impl StripeRequest for RetrieveIdentityVerificationSession {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        let session = &self.session;
        RequestBuilder::new(StripeMethod::Get, format!("/identity/verification_sessions/{session}"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<SessionOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provided_details: Option<ProvidedDetailsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_person: Option<CreateIdentityVerificationSessionRelatedPerson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::ListIdentityVerificationReportType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_flow: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIdentityVerificationSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIdentityVerificationSessionBuilder").finish_non_exhaustive()
    }
}
impl CreateIdentityVerificationSessionBuilder {
    fn new() -> Self {
        Self {
            client_reference_id: None,
            expand: None,
            metadata: None,
            options: None,
            provided_details: None,
            related_customer: None,
            related_customer_account: None,
            related_person: None,
            return_url: None,
            type_: None,
            verification_flow: None,
        }
    }
}
/// Tokens referencing a Person resource and its associated account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIdentityVerificationSessionRelatedPerson {
    /// A token representing a connected account.
    /// If provided, the person parameter is also required and must be associated with the account.
    pub account: String,
    /// A token referencing a Person resource that this verification is being used to verify.
    pub person: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIdentityVerificationSessionRelatedPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIdentityVerificationSessionRelatedPerson").finish_non_exhaustive()
    }
}
impl CreateIdentityVerificationSessionRelatedPerson {
    pub fn new(account: impl Into<String>, person: impl Into<String>) -> Self {
        Self { account: account.into(), person: person.into() }
    }
}
/// Creates a VerificationSession object.
///
/// After the VerificationSession is created, display a verification modal using the session `client_secret` or send your users to the session’s `url`.
///
/// If your API key is in test mode, verification checks won’t actually process, though everything else will occur as if in live mode.
///
/// Related guide: [Verify your users’ identity documents](https://stripe.com/docs/identity/verify-identity-documents).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIdentityVerificationSession {
    inner: CreateIdentityVerificationSessionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIdentityVerificationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIdentityVerificationSession").finish_non_exhaustive()
    }
}
impl CreateIdentityVerificationSession {
    /// Construct a new `CreateIdentityVerificationSession`.
    pub fn new() -> Self {
        Self { inner: CreateIdentityVerificationSessionBuilder::new() }
    }
    /// A string to reference this user.
    /// This can be a customer ID, a session ID, or similar, and can be used to reconcile this verification with your internal systems.
    pub fn client_reference_id(mut self, client_reference_id: impl Into<String>) -> Self {
        self.inner.client_reference_id = Some(client_reference_id.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// A set of options for the session’s verification checks.
    pub fn options(mut self, options: impl Into<SessionOptionsParam>) -> Self {
        self.inner.options = Some(options.into());
        self
    }
    /// Details provided about the user being verified. These details might be shown to the user.
    pub fn provided_details(mut self, provided_details: impl Into<ProvidedDetailsParam>) -> Self {
        self.inner.provided_details = Some(provided_details.into());
        self
    }
    /// Customer ID
    pub fn related_customer(mut self, related_customer: impl Into<String>) -> Self {
        self.inner.related_customer = Some(related_customer.into());
        self
    }
    /// The ID of the Account representing a customer.
    pub fn related_customer_account(mut self, related_customer_account: impl Into<String>) -> Self {
        self.inner.related_customer_account = Some(related_customer_account.into());
        self
    }
    /// Tokens referencing a Person resource and its associated account.
    pub fn related_person(
        mut self,
        related_person: impl Into<CreateIdentityVerificationSessionRelatedPerson>,
    ) -> Self {
        self.inner.related_person = Some(related_person.into());
        self
    }
    /// The URL that the user will be redirected to upon completing the verification flow.
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.inner.return_url = Some(return_url.into());
        self
    }
    /// The type of [verification check](https://docs.stripe.com/identity/verification-checks) to be performed.
    /// You must provide a `type` if not passing `verification_flow`.
    pub fn type_(
        mut self,
        type_: impl Into<stripe_shared::ListIdentityVerificationReportType>,
    ) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
    /// The ID of a verification flow from the Dashboard.
    /// See <https://docs.stripe.com/identity/verification-flows>.
    pub fn verification_flow(mut self, verification_flow: impl Into<String>) -> Self {
        self.inner.verification_flow = Some(verification_flow.into());
        self
    }
}
impl Default for CreateIdentityVerificationSession {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateIdentityVerificationSession {
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

impl StripeRequest for CreateIdentityVerificationSession {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/identity/verification_sessions").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<SessionOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provided_details: Option<ProvidedDetailsParam>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::ListIdentityVerificationReportType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIdentityVerificationSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIdentityVerificationSessionBuilder").finish_non_exhaustive()
    }
}
impl UpdateIdentityVerificationSessionBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None, options: None, provided_details: None, type_: None }
    }
}
/// Updates a VerificationSession object.
///
/// When the session status is `requires_input`, you can use this method to update the
/// verification check and options.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateIdentityVerificationSession {
    inner: UpdateIdentityVerificationSessionBuilder,
    session: stripe_misc::IdentityVerificationSessionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIdentityVerificationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIdentityVerificationSession").finish_non_exhaustive()
    }
}
impl UpdateIdentityVerificationSession {
    /// Construct a new `UpdateIdentityVerificationSession`.
    pub fn new(session: impl Into<stripe_misc::IdentityVerificationSessionId>) -> Self {
        Self { session: session.into(), inner: UpdateIdentityVerificationSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// A set of options for the session’s verification checks.
    pub fn options(mut self, options: impl Into<SessionOptionsParam>) -> Self {
        self.inner.options = Some(options.into());
        self
    }
    /// Details provided about the user being verified. These details may be shown to the user.
    pub fn provided_details(mut self, provided_details: impl Into<ProvidedDetailsParam>) -> Self {
        self.inner.provided_details = Some(provided_details.into());
        self
    }
    /// The type of [verification check](https://docs.stripe.com/identity/verification-checks) to be performed.
    pub fn type_(
        mut self,
        type_: impl Into<stripe_shared::ListIdentityVerificationReportType>,
    ) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl UpdateIdentityVerificationSession {
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

impl StripeRequest for UpdateIdentityVerificationSession {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        let session = &self.session;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/identity/verification_sessions/{session}"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CancelIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancelIdentityVerificationSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancelIdentityVerificationSessionBuilder").finish_non_exhaustive()
    }
}
impl CancelIdentityVerificationSessionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// A VerificationSession object can be canceled when it is in `requires_input` [status](https://stripe.com/docs/identity/how-sessions-work).
///
/// Once canceled, future submission attempts are disabled.
/// This cannot be undone.
/// [Learn more](https://stripe.com/docs/identity/verification-sessions#cancel).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CancelIdentityVerificationSession {
    inner: CancelIdentityVerificationSessionBuilder,
    session: stripe_misc::IdentityVerificationSessionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancelIdentityVerificationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancelIdentityVerificationSession").finish_non_exhaustive()
    }
}
impl CancelIdentityVerificationSession {
    /// Construct a new `CancelIdentityVerificationSession`.
    pub fn new(session: impl Into<stripe_misc::IdentityVerificationSessionId>) -> Self {
        Self { session: session.into(), inner: CancelIdentityVerificationSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelIdentityVerificationSession {
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

impl StripeRequest for CancelIdentityVerificationSession {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        let session = &self.session;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/identity/verification_sessions/{session}/cancel"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RedactIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RedactIdentityVerificationSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RedactIdentityVerificationSessionBuilder").finish_non_exhaustive()
    }
}
impl RedactIdentityVerificationSessionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Redact a VerificationSession to remove all collected information from Stripe. This will redact
/// the VerificationSession and all objects related to it, including VerificationReports, Events,
/// request logs, etc.
///
/// A VerificationSession object can be redacted when it is in `requires_input` or `verified`
/// [status](https://stripe.com/docs/identity/how-sessions-work).
/// Redacting a VerificationSession in `requires_action`.
/// state will automatically cancel it.
///
/// The redaction process may take up to four days. When the redaction process is in progress, the
/// VerificationSession’s `redaction.status` field will be set to `processing`; when the process is
/// finished, it will change to `redacted` and an `identity.verification_session.redacted` event
/// will be emitted.
///
/// Redaction is irreversible. Redacted objects are still accessible in the Stripe API, but all the
/// fields that contain personal data will be replaced by the string `[redacted]` or a similar
/// placeholder. The `metadata` field will also be erased. Redacted objects cannot be updated or
/// used for any purpose.
///
/// [Learn more](https://stripe.com/docs/identity/verification-sessions#redact).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RedactIdentityVerificationSession {
    inner: RedactIdentityVerificationSessionBuilder,
    session: stripe_misc::IdentityVerificationSessionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RedactIdentityVerificationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RedactIdentityVerificationSession").finish_non_exhaustive()
    }
}
impl RedactIdentityVerificationSession {
    /// Construct a new `RedactIdentityVerificationSession`.
    pub fn new(session: impl Into<stripe_misc::IdentityVerificationSessionId>) -> Self {
        Self { session: session.into(), inner: RedactIdentityVerificationSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RedactIdentityVerificationSession {
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

impl StripeRequest for RedactIdentityVerificationSession {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        let session = &self.session;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/identity/verification_sessions/{session}/redact"),
        )
        .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DocumentOptions {
    /// Array of strings of allowed identity document types.
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<stripe_shared::GelatoDocumentReportType>>,
    /// Collect an ID number and perform an [ID number check](https://docs.stripe.com/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,
    /// Disable image uploads, identity document images have to be captured using the device’s camera.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,
    /// Capture a face image and perform a [selfie check](https://docs.stripe.com/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face.
    /// [Learn more](https://docs.stripe.com/identity/selfie).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DocumentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DocumentOptions").finish_non_exhaustive()
    }
}
impl DocumentOptions {
    pub fn new() -> Self {
        Self {
            allowed_types: None,
            require_id_number: None,
            require_live_capture: None,
            require_matching_selfie: None,
        }
    }
}
impl Default for DocumentOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ProvidedDetailsParam {
    /// Email of user being verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number of user being verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ProvidedDetailsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ProvidedDetailsParam").finish_non_exhaustive()
    }
}
impl ProvidedDetailsParam {
    pub fn new() -> Self {
        Self { email: None, phone: None }
    }
}
impl Default for ProvidedDetailsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SessionOptionsParam {
    /// Options that apply to the [document check](https://docs.stripe.com/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SessionOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SessionOptionsParam").finish_non_exhaustive()
    }
}
impl SessionOptionsParam {
    pub fn new() -> Self {
        Self { document: None }
    }
}
impl Default for SessionOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
/// Only return VerificationSessions with this status.
/// [Learn more about the lifecycle of sessions](https://docs.stripe.com/identity/how-sessions-work).
pub type ListIdentityVerificationSessionStatus = stripe_shared::IdentityVerificationSessionStatus;
/// Array of strings of allowed identity document types.
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
pub type CreateIdentityVerificationSessionOptionsDocumentAllowedTypes =
    stripe_shared::GelatoDocumentReportType;
/// The type of [verification check](https://docs.stripe.com/identity/verification-checks) to be performed.
/// You must provide a `type` if not passing `verification_flow`.
pub type CreateIdentityVerificationSessionType = stripe_shared::ListIdentityVerificationReportType;
/// Array of strings of allowed identity document types.
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
pub type UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes =
    stripe_shared::GelatoDocumentReportType;
/// The type of [verification check](https://docs.stripe.com/identity/verification-checks) to be performed.
pub type UpdateIdentityVerificationSessionType = stripe_shared::ListIdentityVerificationReportType;
/// Options that apply to the [document check](https://docs.stripe.com/identity/verification-checks?type=document).
pub type CreateIdentityVerificationSessionOptionsDocument = DocumentOptions;
/// Details provided about the user being verified. These details might be shown to the user.
pub type CreateIdentityVerificationSessionProvidedDetails = ProvidedDetailsParam;
/// Options that apply to the [document check](https://docs.stripe.com/identity/verification-checks?type=document).
pub type UpdateIdentityVerificationSessionOptionsDocument = DocumentOptions;
/// Details provided about the user being verified. These details may be shown to the user.
pub type UpdateIdentityVerificationSessionProvidedDetails = ProvidedDetailsParam;
/// A set of options for the session’s verification checks.
pub type CreateIdentityVerificationSessionOptions = SessionOptionsParam;
/// A set of options for the session’s verification checks.
pub type UpdateIdentityVerificationSessionOptions = SessionOptionsParam;
