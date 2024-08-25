use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
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
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_misc::IdentityVerificationSessionStatus>,
}
impl ListIdentityVerificationSessionBuilder {
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
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListIdentityVerificationSession {
    inner: ListIdentityVerificationSessionBuilder,
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return VerificationSessions with this status.
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub fn status(
        mut self,
        status: impl Into<stripe_misc::IdentityVerificationSessionStatus>,
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveIdentityVerificationSession {
    inner: RetrieveIdentityVerificationSessionBuilder,
    session: stripe_misc::IdentityVerificationSessionId,
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<CreateIdentityVerificationSessionOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provided_details: Option<ProvidedDetailsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<CreateIdentityVerificationSessionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_flow: Option<String>,
}
impl CreateIdentityVerificationSessionBuilder {
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateIdentityVerificationSessionOptions {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateIdentityVerificationSessionOptionsDocument>,
}
impl CreateIdentityVerificationSessionOptions {
    pub fn new() -> Self {
        Self { document: None }
    }
}
impl Default for CreateIdentityVerificationSessionOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateIdentityVerificationSessionOptionsDocument {
    /// Array of strings of allowed identity document types.
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<CreateIdentityVerificationSessionOptionsDocumentAllowedTypes>>,
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
impl CreateIdentityVerificationSessionOptionsDocument {
    pub fn new() -> Self {
        Self {
            allowed_types: None,
            require_id_number: None,
            require_live_capture: None,
            require_matching_selfie: None,
        }
    }
}
impl Default for CreateIdentityVerificationSessionOptionsDocument {
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
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateIdentityVerificationSession {
    inner: CreateIdentityVerificationSessionBuilder,
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    pub fn options(mut self, options: impl Into<CreateIdentityVerificationSessionOptions>) -> Self {
        self.inner.options = Some(options.into());
        self
    }
    /// Details provided about the user being verified. These details may be shown to the user.
    pub fn provided_details(mut self, provided_details: impl Into<ProvidedDetailsParam>) -> Self {
        self.inner.provided_details = Some(provided_details.into());
        self
    }
    /// The URL that the user will be redirected to upon completing the verification flow.
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.inner.return_url = Some(return_url.into());
        self
    }
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    /// You must provide a `type` if not passing `verification_flow`.
    pub fn type_(mut self, type_: impl Into<CreateIdentityVerificationSessionType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
    /// The ID of a Verification Flow from the Dashboard.
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct UpdateIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<UpdateIdentityVerificationSessionOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provided_details: Option<ProvidedDetailsParam>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<UpdateIdentityVerificationSessionType>,
}
impl UpdateIdentityVerificationSessionBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None, options: None, provided_details: None, type_: None }
    }
}
/// A set of options for the session’s verification checks.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateIdentityVerificationSessionOptions {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<UpdateIdentityVerificationSessionOptionsDocument>,
}
impl UpdateIdentityVerificationSessionOptions {
    pub fn new() -> Self {
        Self { document: None }
    }
}
impl Default for UpdateIdentityVerificationSessionOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateIdentityVerificationSessionOptionsDocument {
    /// Array of strings of allowed identity document types.
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<UpdateIdentityVerificationSessionOptionsDocumentAllowedTypes>>,
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
impl UpdateIdentityVerificationSessionOptionsDocument {
    pub fn new() -> Self {
        Self {
            allowed_types: None,
            require_id_number: None,
            require_live_capture: None,
            require_matching_selfie: None,
        }
    }
}
impl Default for UpdateIdentityVerificationSessionOptionsDocument {
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
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateIdentityVerificationSession {
    inner: UpdateIdentityVerificationSessionBuilder,
    session: stripe_misc::IdentityVerificationSessionId,
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    pub fn options(mut self, options: impl Into<UpdateIdentityVerificationSessionOptions>) -> Self {
        self.inner.options = Some(options.into());
        self
    }
    /// Details provided about the user being verified. These details may be shown to the user.
    pub fn provided_details(mut self, provided_details: impl Into<ProvidedDetailsParam>) -> Self {
        self.inner.provided_details = Some(provided_details.into());
        self
    }
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    pub fn type_(mut self, type_: impl Into<UpdateIdentityVerificationSessionType>) -> Self {
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CancelIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CancelIdentityVerificationSession {
    inner: CancelIdentityVerificationSessionBuilder,
    session: stripe_misc::IdentityVerificationSessionId,
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RedactIdentityVerificationSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RedactIdentityVerificationSession {
    inner: RedactIdentityVerificationSessionBuilder,
    session: stripe_misc::IdentityVerificationSessionId,
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

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ProvidedDetailsParam {
    /// Email of user being verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number of user being verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
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
