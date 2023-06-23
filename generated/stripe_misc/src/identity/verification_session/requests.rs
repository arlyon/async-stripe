use stripe::{Client, Response};

impl stripe_misc::identity::verification_session::VerificationSession {
    /// Creates a VerificationSession object.
    ///
    /// After the VerificationSession is created, display a verification modal using the session `client_secret` or send your users to the session’s `url`.
    ///
    /// If your API key is in test mode, verification checks won’t actually process, though everything else will occur as if in live mode.
    ///
    /// Related guide: [Verify your users’ identity documents](https://stripe.com/docs/identity/verify-identity-documents).
    pub fn create(
        client: &Client,
        params: CreateVerificationSession,
    ) -> Response<stripe_misc::identity::verification_session::VerificationSession> {
        client.send_form("/identity/verification_sessions", params, http_types::Method::Post)
    }
    /// Retrieves the details of a VerificationSession that was previously created.
    ///
    /// When the session status is `requires_input`, you can use this method to retrieve a valid
    /// `client_secret` or `url` to allow re-submission.
    pub fn retrieve(
        client: &Client,
        session: &str,
        params: RetrieveVerificationSession,
    ) -> Response<stripe_misc::identity::verification_session::VerificationSession> {
        client.get_query(
            &format!("/identity/verification_sessions/{session}", session = session),
            params,
        )
    }
    /// Returns a list of VerificationSessions.
    pub fn list(
        client: &Client,
        params: ListVerificationSession,
    ) -> Response<
        stripe_types::List<stripe_misc::identity::verification_session::VerificationSession>,
    > {
        client.get_query("/identity/verification_sessions", params)
    }
    /// A VerificationSession object can be canceled when it is in `requires_input` [status](https://stripe.com/docs/identity/how-sessions-work).
    ///
    /// Once canceled, future submission attempts are disabled.
    ///
    /// This cannot be undone.
    /// [Learn more](https://stripe.com/docs/identity/verification-sessions#cancel).
    pub fn cancel(
        client: &Client,
        session: &str,
        params: CancelVerificationSession,
    ) -> Response<stripe_misc::identity::verification_session::VerificationSession> {
        client.send_form(
            &format!("/identity/verification_sessions/{session}/cancel", session = session),
            params,
            http_types::Method::Post,
        )
    }
    /// Redact a VerificationSession to remove all collected information from Stripe.
    ///
    /// This will redact the VerificationSession and all objects related to it, including VerificationReports, Events, request logs, etc.  A VerificationSession object can be redacted when it is in `requires_input` or `verified` [status](https://stripe.com/docs/identity/how-sessions-work).
    /// Redacting a VerificationSession in `requires_action` state will automatically cancel it.  The redaction process may take up to four days.
    /// When the redaction process is in progress, the VerificationSession’s `redaction.status` field will be set to `processing`; when the process is finished, it will change to `redacted` and an `identity.verification_session.redacted` event will be emitted.  Redaction is irreversible.
    /// Redacted objects are still accessible in the Stripe API, but all the fields that contain personal data will be replaced by the string `[redacted]` or a similar placeholder.
    /// The `metadata` field will also be erased.
    /// Redacted objects cannot be updated or used for any purpose.  [Learn more](https://stripe.com/docs/identity/verification-sessions#redact).
    pub fn redact(
        client: &Client,
        session: &str,
        params: RedactVerificationSession,
    ) -> Response<stripe_misc::identity::verification_session::VerificationSession> {
        client.send_form(
            &format!("/identity/verification_sessions/{session}/redact", session = session),
            params,
            http_types::Method::Post,
        )
    }
    /// Updates a VerificationSession object.
    ///
    /// When the session status is `requires_input`, you can use this method to update the
    /// verification check and options.
    pub fn update(
        client: &Client,
        session: &str,
        params: UpdateVerificationSession,
    ) -> Response<stripe_misc::identity::verification_session::VerificationSession> {
        client.send_form(
            &format!("/identity/verification_sessions/{session}", session = session),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A set of options for the session’s verification checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateVerificationSessionOptions<'a>>,
    /// The URL that the user will be redirected to upon completing the verification flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    pub type_: CreateVerificationSessionType,
}
impl<'a> CreateVerificationSession<'a> {
    pub fn new(type_: CreateVerificationSessionType) -> Self {
        Self {
            expand: Default::default(),
            metadata: Default::default(),
            options: Default::default(),
            return_url: Default::default(),
            type_,
        }
    }
}
/// A set of options for the session’s verification checks.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateVerificationSessionOptions<'a> {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateVerificationSessionOptionsDocument<'a>>,
}
impl<'a> CreateVerificationSessionOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateVerificationSessionOptionsDocument<'a> {
    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<&'a [CreateVerificationSessionOptionsDocumentAllowedTypes]>,
    /// Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,
    /// Disable image uploads, identity document images have to be captured using the device’s camera.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,
    /// Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face.
    ///
    /// [Learn more](https://stripe.com/docs/identity/selfie).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}
impl<'a> CreateVerificationSessionOptionsDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Array of strings of allowed identity document types.
///
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateVerificationSessionOptionsDocumentAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl CreateVerificationSessionOptionsDocumentAllowedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DrivingLicense => "driving_license",
            Self::IdCard => "id_card",
            Self::Passport => "passport",
        }
    }
}

impl AsRef<str> for CreateVerificationSessionOptionsDocumentAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateVerificationSessionType {
    Document,
    IdNumber,
}

impl CreateVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::IdNumber => "id_number",
        }
    }
}

impl AsRef<str> for CreateVerificationSessionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListVerificationSession<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return VerificationSessions with this status.
    ///
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListVerificationSessionStatus>,
}
impl<'a> ListVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return VerificationSessions with this status.
///
/// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListVerificationSessionStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
}

impl ListVerificationSessionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Processing => "processing",
            Self::RequiresInput => "requires_input",
            Self::Verified => "verified",
        }
    }
}

impl AsRef<str> for ListVerificationSessionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListVerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RedactVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RedactVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A set of options for the session’s verification checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<UpdateVerificationSessionOptions<'a>>,
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateVerificationSessionType>,
}
impl<'a> UpdateVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A set of options for the session’s verification checks.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateVerificationSessionOptions<'a> {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<UpdateVerificationSessionOptionsDocument<'a>>,
}
impl<'a> UpdateVerificationSessionOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateVerificationSessionOptionsDocument<'a> {
    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<&'a [UpdateVerificationSessionOptionsDocumentAllowedTypes]>,
    /// Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,
    /// Disable image uploads, identity document images have to be captured using the device’s camera.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,
    /// Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face.
    ///
    /// [Learn more](https://stripe.com/docs/identity/selfie).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}
impl<'a> UpdateVerificationSessionOptionsDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Array of strings of allowed identity document types.
///
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateVerificationSessionOptionsDocumentAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl UpdateVerificationSessionOptionsDocumentAllowedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DrivingLicense => "driving_license",
            Self::IdCard => "id_card",
            Self::Passport => "passport",
        }
    }
}

impl AsRef<str> for UpdateVerificationSessionOptionsDocumentAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateVerificationSessionType {
    Document,
    IdNumber,
}

impl UpdateVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::IdNumber => "id_number",
        }
    }
}

impl AsRef<str> for UpdateVerificationSessionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
