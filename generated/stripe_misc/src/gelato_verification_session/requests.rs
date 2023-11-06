#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateGelatoVerificationSession<'a> {
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
    pub options: Option<CreateGelatoVerificationSessionOptions<'a>>,
    /// The URL that the user will be redirected to upon completing the verification flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    pub type_: CreateGelatoVerificationSessionType,
}
impl<'a> CreateGelatoVerificationSession<'a> {
    pub fn new(type_: CreateGelatoVerificationSessionType) -> Self {
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
pub struct CreateGelatoVerificationSessionOptions<'a> {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateGelatoVerificationSessionOptionsDocument<'a>>,
}
impl<'a> CreateGelatoVerificationSessionOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateGelatoVerificationSessionOptionsDocument<'a> {
    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<&'a [CreateGelatoVerificationSessionOptionsDocumentAllowedTypes]>,
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
impl<'a> CreateGelatoVerificationSessionOptionsDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Array of strings of allowed identity document types.
///
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl CreateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    pub fn as_str(self) -> &'static str {
        use CreateGelatoVerificationSessionOptionsDocumentAllowedTypes::*;
        match self {
            DrivingLicense => "driving_license",
            IdCard => "id_card",
            Passport => "passport",
        }
    }
}

impl std::str::FromStr for CreateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateGelatoVerificationSessionOptionsDocumentAllowedTypes::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateGelatoVerificationSessionType {
    Document,
    IdNumber,
}

impl CreateGelatoVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        use CreateGelatoVerificationSessionType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for CreateGelatoVerificationSessionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateGelatoVerificationSessionType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateGelatoVerificationSessionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateGelatoVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateGelatoVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateGelatoVerificationSessionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateGelatoVerificationSession<'a> {
    /// Creates a VerificationSession object.
    ///
    /// After the VerificationSession is created, display a verification modal using the session `client_secret` or send your users to the session’s `url`.
    ///
    /// If your API key is in test mode, verification checks won’t actually process, though everything else will occur as if in live mode.
    ///
    /// Related guide: [Verify your users’ identity documents](https://stripe.com/docs/identity/verify-identity-documents).
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_misc::GelatoVerificationSession> {
        client.send_form("/identity/verification_sessions", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveGelatoVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveGelatoVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveGelatoVerificationSession<'a> {
    /// Retrieves the details of a VerificationSession that was previously created.
    ///
    /// When the session status is `requires_input`, you can use this method to retrieve a valid
    /// `client_secret` or `url` to allow re-submission.
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_misc::gelato_verification_session::IdentityVerificationSessionId,
    ) -> stripe::Response<stripe_misc::GelatoVerificationSession> {
        client.get_query(
            &format!("/identity/verification_sessions/{session}", session = session),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListGelatoVerificationSession<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return VerificationSessions with this status.
    ///
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListGelatoVerificationSessionStatus>,
}
impl<'a> ListGelatoVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return VerificationSessions with this status.
///
/// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListGelatoVerificationSessionStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
}

impl ListGelatoVerificationSessionStatus {
    pub fn as_str(self) -> &'static str {
        use ListGelatoVerificationSessionStatus::*;
        match self {
            Canceled => "canceled",
            Processing => "processing",
            RequiresInput => "requires_input",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for ListGelatoVerificationSessionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListGelatoVerificationSessionStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "processing" => Ok(Processing),
            "requires_input" => Ok(RequiresInput),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListGelatoVerificationSessionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListGelatoVerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListGelatoVerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListGelatoVerificationSessionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListGelatoVerificationSession<'a> {
    /// Returns a list of VerificationSessions.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::GelatoVerificationSession>> {
        client.get_query("/identity/verification_sessions", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_misc::GelatoVerificationSession> {
        stripe::ListPaginator::from_params("/identity/verification_sessions", self)
    }
}
impl<'a> stripe::PaginationParams for ListGelatoVerificationSession<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelGelatoVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelGelatoVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelGelatoVerificationSession<'a> {
    /// A VerificationSession object can be canceled when it is in `requires_input` [status](https://stripe.com/docs/identity/how-sessions-work).
    ///
    /// Once canceled, future submission attempts are disabled.
    ///
    /// This cannot be undone.
    /// [Learn more](https://stripe.com/docs/identity/verification-sessions#cancel).
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_misc::gelato_verification_session::IdentityVerificationSessionId,
    ) -> stripe::Response<stripe_misc::GelatoVerificationSession> {
        client.send_form(
            &format!("/identity/verification_sessions/{session}/cancel", session = session),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RedactGelatoVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RedactGelatoVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RedactGelatoVerificationSession<'a> {
    /// Redact a VerificationSession to remove all collected information from Stripe.
    ///
    /// This will redact the VerificationSession and all objects related to it, including VerificationReports, Events, request logs, etc.  A VerificationSession object can be redacted when it is in `requires_input` or `verified` [status](https://stripe.com/docs/identity/how-sessions-work).
    /// Redacting a VerificationSession in `requires_action` state will automatically cancel it.  The redaction process may take up to four days.
    /// When the redaction process is in progress, the VerificationSession’s `redaction.status` field will be set to `processing`; when the process is finished, it will change to `redacted` and an `identity.verification_session.redacted` event will be emitted.  Redaction is irreversible.
    /// Redacted objects are still accessible in the Stripe API, but all the fields that contain personal data will be replaced by the string `[redacted]` or a similar placeholder.
    /// The `metadata` field will also be erased.
    /// Redacted objects cannot be updated or used for any purpose.  [Learn more](https://stripe.com/docs/identity/verification-sessions#redact).
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_misc::gelato_verification_session::IdentityVerificationSessionId,
    ) -> stripe::Response<stripe_misc::GelatoVerificationSession> {
        client.send_form(
            &format!("/identity/verification_sessions/{session}/redact", session = session),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateGelatoVerificationSession<'a> {
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
    pub options: Option<UpdateGelatoVerificationSessionOptions<'a>>,
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateGelatoVerificationSessionType>,
}
impl<'a> UpdateGelatoVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A set of options for the session’s verification checks.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateGelatoVerificationSessionOptions<'a> {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<UpdateGelatoVerificationSessionOptionsDocument<'a>>,
}
impl<'a> UpdateGelatoVerificationSessionOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateGelatoVerificationSessionOptionsDocument<'a> {
    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<&'a [UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes]>,
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
impl<'a> UpdateGelatoVerificationSessionOptionsDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Array of strings of allowed identity document types.
///
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    pub fn as_str(self) -> &'static str {
        use UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes::*;
        match self {
            DrivingLicense => "driving_license",
            IdCard => "id_card",
            Passport => "passport",
        }
    }
}

impl std::str::FromStr for UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateGelatoVerificationSessionOptionsDocumentAllowedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateGelatoVerificationSessionType {
    Document,
    IdNumber,
}

impl UpdateGelatoVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        use UpdateGelatoVerificationSessionType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for UpdateGelatoVerificationSessionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateGelatoVerificationSessionType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateGelatoVerificationSessionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateGelatoVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateGelatoVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateGelatoVerificationSessionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> UpdateGelatoVerificationSession<'a> {
    /// Updates a VerificationSession object.
    ///
    /// When the session status is `requires_input`, you can use this method to update the
    /// verification check and options.
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_misc::gelato_verification_session::IdentityVerificationSessionId,
    ) -> stripe::Response<stripe_misc::GelatoVerificationSession> {
        client.send_form(
            &format!("/identity/verification_sessions/{session}", session = session),
            self,
            http_types::Method::Post,
        )
    }
}
