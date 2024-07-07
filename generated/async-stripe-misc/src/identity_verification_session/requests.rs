use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListIdentityVerificationSessionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_reference_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_misc::IdentityVerificationSessionStatus>,
}
impl<'a> ListIdentityVerificationSessionBuilder<'a> {
    fn new() -> Self {
        Self {
            client_reference_id: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of VerificationSessions
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIdentityVerificationSession<'a> {
    inner: ListIdentityVerificationSessionBuilder<'a>,
}
impl<'a> ListIdentityVerificationSession<'a> {
    /// Construct a new `ListIdentityVerificationSession`.
    pub fn new() -> Self {
        Self { inner: ListIdentityVerificationSessionBuilder::new() }
    }
    /// A string to reference this user.
    /// This can be a customer ID, a session ID, or similar, and can be used to reconcile this verification with your internal systems.
    pub fn client_reference_id(mut self, client_reference_id: &'a str) -> Self {
        self.inner.client_reference_id = Some(client_reference_id);
        self
    }
    /// Only return VerificationSessions that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
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
    /// Only return VerificationSessions with this status.
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub fn status(mut self, status: stripe_misc::IdentityVerificationSessionStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
}
impl<'a> Default for ListIdentityVerificationSession<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIdentityVerificationSession<'_> {
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
        stripe_client_core::ListPaginator::new_list("/identity/verification_sessions", self.inner)
    }
}

impl StripeRequest for ListIdentityVerificationSession<'_> {
    type Output = stripe_types::List<stripe_misc::IdentityVerificationSession>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/identity/verification_sessions").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveIdentityVerificationSessionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIdentityVerificationSessionBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a VerificationSession that was previously created.
///
/// When the session status is `requires_input`, you can use this method to retrieve a valid
/// `client_secret` or `url` to allow re-submission.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveIdentityVerificationSession<'a> {
    inner: RetrieveIdentityVerificationSessionBuilder<'a>,
    session: &'a stripe_misc::IdentityVerificationSessionId,
}
impl<'a> RetrieveIdentityVerificationSession<'a> {
    /// Construct a new `RetrieveIdentityVerificationSession`.
    pub fn new(session: &'a stripe_misc::IdentityVerificationSessionId) -> Self {
        Self { session, inner: RetrieveIdentityVerificationSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveIdentityVerificationSession<'_> {
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

impl StripeRequest for RetrieveIdentityVerificationSession<'_> {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        let session = self.session;
        RequestBuilder::new(StripeMethod::Get, format!("/identity/verification_sessions/{session}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateIdentityVerificationSessionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_reference_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<CreateIdentityVerificationSessionOptions<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provided_details: Option<ProvidedDetailsParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<&'a str>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<CreateIdentityVerificationSessionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_flow: Option<&'a str>,
}
impl<'a> CreateIdentityVerificationSessionBuilder<'a> {
    fn new() -> Self {
        Self {
            client_reference_id: None,
            expand: None,
            metadata: None,
            options: None,
            provided_details: None,
            return_url: None,
            type_: None,
            verification_flow: None,
        }
    }
}
/// A set of options for the session’s verification checks.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateIdentityVerificationSessionOptions<'a> {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateIdentityVerificationSessionOptionsDocument<'a>>,
}
impl<'a> CreateIdentityVerificationSessionOptions<'a> {
    pub fn new() -> Self {
        Self { document: None }
    }
}
impl<'a> Default for CreateIdentityVerificationSessionOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
        Self {
            allowed_types: None,
            require_id_number: None,
            require_live_capture: None,
            require_matching_selfie: None,
        }
    }
}
impl<'a> Default for CreateIdentityVerificationSessionOptionsDocument<'a> {
    fn default() -> Self {
        Self::new()
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIdentityVerificationSessionOptionsDocumentAllowedTypes::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateIdentityVerificationSessionOptionsDocumentAllowedTypes",
            )
        })
    }
}
/// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
/// You must provide a `type` if not passing `verification_flow`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIdentityVerificationSessionType {
    Document,
    IdNumber,
}
impl CreateIdentityVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        use CreateIdentityVerificationSessionType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for CreateIdentityVerificationSessionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIdentityVerificationSessionType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateIdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIdentityVerificationSessionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateIdentityVerificationSessionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateIdentityVerificationSessionType")
        })
    }
}
/// Creates a VerificationSession object.
///
/// After the VerificationSession is created, display a verification modal using the session `client_secret` or send your users to the session’s `url`.
///
/// If your API key is in test mode, verification checks won’t actually process, though everything else will occur as if in live mode.
///
/// Related guide: [Verify your users’ identity documents](https://stripe.com/docs/identity/verify-identity-documents).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateIdentityVerificationSession<'a> {
    inner: CreateIdentityVerificationSessionBuilder<'a>,
}
impl<'a> CreateIdentityVerificationSession<'a> {
    /// Construct a new `CreateIdentityVerificationSession`.
    pub fn new() -> Self {
        Self { inner: CreateIdentityVerificationSessionBuilder::new() }
    }
    /// A string to reference this user.
    /// This can be a customer ID, a session ID, or similar, and can be used to reconcile this verification with your internal systems.
    pub fn client_reference_id(mut self, client_reference_id: &'a str) -> Self {
        self.inner.client_reference_id = Some(client_reference_id);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// A set of options for the session’s verification checks.
    pub fn options(mut self, options: CreateIdentityVerificationSessionOptions<'a>) -> Self {
        self.inner.options = Some(options);
        self
    }
    /// Details provided about the user being verified. These details may be shown to the user.
    pub fn provided_details(mut self, provided_details: ProvidedDetailsParam<'a>) -> Self {
        self.inner.provided_details = Some(provided_details);
        self
    }
    /// The URL that the user will be redirected to upon completing the verification flow.
    pub fn return_url(mut self, return_url: &'a str) -> Self {
        self.inner.return_url = Some(return_url);
        self
    }
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    /// You must provide a `type` if not passing `verification_flow`.
    pub fn type_(mut self, type_: CreateIdentityVerificationSessionType) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
    /// The ID of a Verification Flow from the Dashboard.
    /// See <https://docs.stripe.com/identity/verification-flows>.
    pub fn verification_flow(mut self, verification_flow: &'a str) -> Self {
        self.inner.verification_flow = Some(verification_flow);
        self
    }
}
impl<'a> Default for CreateIdentityVerificationSession<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateIdentityVerificationSession<'_> {
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

impl StripeRequest for CreateIdentityVerificationSession<'_> {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/identity/verification_sessions").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateIdentityVerificationSessionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<UpdateIdentityVerificationSessionOptions<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provided_details: Option<ProvidedDetailsParam<'a>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<UpdateIdentityVerificationSessionType>,
}
impl<'a> UpdateIdentityVerificationSessionBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, metadata: None, options: None, provided_details: None, type_: None }
    }
}
/// A set of options for the session’s verification checks.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateIdentityVerificationSessionOptions<'a> {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<UpdateIdentityVerificationSessionOptionsDocument<'a>>,
}
impl<'a> UpdateIdentityVerificationSessionOptions<'a> {
    pub fn new() -> Self {
        Self { document: None }
    }
}
impl<'a> Default for UpdateIdentityVerificationSessionOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
        Self {
            allowed_types: None,
            require_id_number: None,
            require_live_capture: None,
            require_matching_selfie: None,
        }
    }
}
impl<'a> Default for UpdateIdentityVerificationSessionOptionsDocument<'a> {
    fn default() -> Self {
        Self::new()
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes",
            )
        })
    }
}
/// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIdentityVerificationSessionType {
    Document,
    IdNumber,
}
impl UpdateIdentityVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        use UpdateIdentityVerificationSessionType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for UpdateIdentityVerificationSessionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIdentityVerificationSessionType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateIdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIdentityVerificationSessionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateIdentityVerificationSessionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateIdentityVerificationSessionType")
        })
    }
}
/// Updates a VerificationSession object.
///
/// When the session status is `requires_input`, you can use this method to update the
/// verification check and options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateIdentityVerificationSession<'a> {
    inner: UpdateIdentityVerificationSessionBuilder<'a>,
    session: &'a stripe_misc::IdentityVerificationSessionId,
}
impl<'a> UpdateIdentityVerificationSession<'a> {
    /// Construct a new `UpdateIdentityVerificationSession`.
    pub fn new(session: &'a stripe_misc::IdentityVerificationSessionId) -> Self {
        Self { session, inner: UpdateIdentityVerificationSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// A set of options for the session’s verification checks.
    pub fn options(mut self, options: UpdateIdentityVerificationSessionOptions<'a>) -> Self {
        self.inner.options = Some(options);
        self
    }
    /// Details provided about the user being verified. These details may be shown to the user.
    pub fn provided_details(mut self, provided_details: ProvidedDetailsParam<'a>) -> Self {
        self.inner.provided_details = Some(provided_details);
        self
    }
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    pub fn type_(mut self, type_: UpdateIdentityVerificationSessionType) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
}
impl UpdateIdentityVerificationSession<'_> {
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

impl StripeRequest for UpdateIdentityVerificationSession<'_> {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        let session = self.session;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/identity/verification_sessions/{session}"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CancelIdentityVerificationSessionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CancelIdentityVerificationSessionBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// A VerificationSession object can be canceled when it is in `requires_input` [status](https://stripe.com/docs/identity/how-sessions-work).
///
/// Once canceled, future submission attempts are disabled.
/// This cannot be undone.
/// [Learn more](https://stripe.com/docs/identity/verification-sessions#cancel).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelIdentityVerificationSession<'a> {
    inner: CancelIdentityVerificationSessionBuilder<'a>,
    session: &'a stripe_misc::IdentityVerificationSessionId,
}
impl<'a> CancelIdentityVerificationSession<'a> {
    /// Construct a new `CancelIdentityVerificationSession`.
    pub fn new(session: &'a stripe_misc::IdentityVerificationSessionId) -> Self {
        Self { session, inner: CancelIdentityVerificationSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CancelIdentityVerificationSession<'_> {
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

impl StripeRequest for CancelIdentityVerificationSession<'_> {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        let session = self.session;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/identity/verification_sessions/{session}/cancel"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RedactIdentityVerificationSessionBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RedactIdentityVerificationSessionBuilder<'a> {
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct RedactIdentityVerificationSession<'a> {
    inner: RedactIdentityVerificationSessionBuilder<'a>,
    session: &'a stripe_misc::IdentityVerificationSessionId,
}
impl<'a> RedactIdentityVerificationSession<'a> {
    /// Construct a new `RedactIdentityVerificationSession`.
    pub fn new(session: &'a stripe_misc::IdentityVerificationSessionId) -> Self {
        Self { session, inner: RedactIdentityVerificationSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RedactIdentityVerificationSession<'_> {
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

impl StripeRequest for RedactIdentityVerificationSession<'_> {
    type Output = stripe_misc::IdentityVerificationSession;

    fn build(&self) -> RequestBuilder {
        let session = self.session;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/identity/verification_sessions/{session}/redact"),
        )
        .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ProvidedDetailsParam<'a> {
    /// Email of user being verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Phone number of user being verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> ProvidedDetailsParam<'a> {
    pub fn new() -> Self {
        Self { email: None, phone: None }
    }
}
impl<'a> Default for ProvidedDetailsParam<'a> {
    fn default() -> Self {
        Self::new()
    }
}
