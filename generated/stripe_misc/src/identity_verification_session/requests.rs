#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListIdentityVerificationSession<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return VerificationSessions with this status.
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_misc::IdentityVerificationSessionStatus>,
}
impl<'a> ListIdentityVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListIdentityVerificationSession<'a> {
    /// Returns a list of VerificationSessions
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::IdentityVerificationSession>> {
        client.get_query("/identity/verification_sessions", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::IdentityVerificationSession>> {
        stripe::ListPaginator::from_list_params("/identity/verification_sessions", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveIdentityVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIdentityVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveIdentityVerificationSession<'a> {
    /// Retrieves the details of a VerificationSession that was previously created.
    ///
    /// When the session status is `requires_input`, you can use this method to retrieve a valid
    /// `client_secret` or `url` to allow re-submission.
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_misc::IdentityVerificationSessionId,
    ) -> stripe::Response<stripe_misc::IdentityVerificationSession> {
        client.get_query(&format!("/identity/verification_sessions/{session}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateIdentityVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A set of options for the session’s verification checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateIdentityVerificationSessionOptions<'a>>,
    /// The URL that the user will be redirected to upon completing the verification flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    pub type_: stripe_misc::IdentityVerificationSessionType,
}
impl<'a> CreateIdentityVerificationSession<'a> {
    pub fn new(type_: stripe_misc::IdentityVerificationSessionType) -> Self {
        Self { expand: None, metadata: None, options: None, return_url: None, type_ }
    }
}
/// A set of options for the session’s verification checks.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIdentityVerificationSessionOptions<'a> {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateIdentityVerificationSessionOptionsDocument<'a>>,
}
impl<'a> CreateIdentityVerificationSessionOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIdentityVerificationSessionOptionsDocument<'a> {
    /// Array of strings of allowed identity document types.
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<&'a [CreateIdentityVerificationSessionOptionsDocumentAllowedTypes]>,
    /// Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,
    /// Disable image uploads, identity document images have to be captured using the device’s camera.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,
    /// Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face.
    /// [Learn more](https://stripe.com/docs/identity/selfie).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}
impl<'a> CreateIdentityVerificationSessionOptionsDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Array of strings of allowed identity document types.
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}
impl CreateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    pub fn as_str(self) -> &'static str {
        use CreateIdentityVerificationSessionOptionsDocumentAllowedTypes::*;
        match self {
            DrivingLicense => "driving_license",
            IdCard => "id_card",
            Passport => "passport",
        }
    }
}

impl std::str::FromStr for CreateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIdentityVerificationSessionOptionsDocumentAllowedTypes::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateIdentityVerificationSession<'a> {
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
    ) -> stripe::Response<stripe_misc::IdentityVerificationSession> {
        client.send_form("/identity/verification_sessions", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIdentityVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A set of options for the session’s verification checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<UpdateIdentityVerificationSessionOptions<'a>>,
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_misc::IdentityVerificationSessionType>,
}
impl<'a> UpdateIdentityVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A set of options for the session’s verification checks.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIdentityVerificationSessionOptions<'a> {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<UpdateIdentityVerificationSessionOptionsDocument<'a>>,
}
impl<'a> UpdateIdentityVerificationSessionOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIdentityVerificationSessionOptionsDocument<'a> {
    /// Array of strings of allowed identity document types.
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<&'a [UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes]>,
    /// Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,
    /// Disable image uploads, identity document images have to be captured using the device’s camera.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,
    /// Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face.
    /// [Learn more](https://stripe.com/docs/identity/selfie).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}
impl<'a> UpdateIdentityVerificationSessionOptionsDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Array of strings of allowed identity document types.
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}
impl UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    pub fn as_str(self) -> &'static str {
        use UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes::*;
        match self {
            DrivingLicense => "driving_license",
            IdCard => "id_card",
            Passport => "passport",
        }
    }
}

impl std::str::FromStr for UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> UpdateIdentityVerificationSession<'a> {
    /// Updates a VerificationSession object.
    ///
    /// When the session status is `requires_input`, you can use this method to update the
    /// verification check and options.
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_misc::IdentityVerificationSessionId,
    ) -> stripe::Response<stripe_misc::IdentityVerificationSession> {
        client.send_form(
            &format!("/identity/verification_sessions/{session}"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelIdentityVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelIdentityVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelIdentityVerificationSession<'a> {
    /// A VerificationSession object can be canceled when it is in `requires_input` [status](https://stripe.com/docs/identity/how-sessions-work).
    ///
    /// Once canceled, future submission attempts are disabled.
    /// This cannot be undone.
    /// [Learn more](https://stripe.com/docs/identity/verification-sessions#cancel).
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_misc::IdentityVerificationSessionId,
    ) -> stripe::Response<stripe_misc::IdentityVerificationSession> {
        client.send_form(
            &format!("/identity/verification_sessions/{session}/cancel"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RedactIdentityVerificationSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RedactIdentityVerificationSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RedactIdentityVerificationSession<'a> {
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
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_misc::IdentityVerificationSessionId,
    ) -> stripe::Response<stripe_misc::IdentityVerificationSession> {
        client.send_form(
            &format!("/identity/verification_sessions/{session}/redact"),
            self,
            http_types::Method::Post,
        )
    }
}
