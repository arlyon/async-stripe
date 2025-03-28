// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{IdentityVerificationSessionId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Address, IdentityVerificationReport};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "GelatoVerificationSession".
///
/// For more details see <https://stripe.com/docs/api/identity/verification_sessions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IdentityVerificationSession {
    /// Unique identifier for the object.
    pub id: IdentityVerificationSessionId,

    /// A string to reference this user.
    ///
    /// This can be a customer ID, a session ID, or similar, and can be used to reconcile this verification with your internal systems.
    pub client_reference_id: Option<String>,

    /// The short-lived client secret used by Stripe.js to [show a verification modal](https://stripe.com/docs/js/identity/modal) inside your app.
    ///
    /// This client secret expires after 24 hours and can only be used once.
    /// Don’t store it, log it, embed it in a URL, or expose it to anyone other than the user.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    /// Refer to our docs on [passing the client secret to the frontend](https://stripe.com/docs/identity/verification-sessions#client-secret) to learn more.
    pub client_secret: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// If present, this property tells you the last error encountered when processing the verification.
    pub last_error: Option<GelatoSessionLastError>,

    /// ID of the most recent VerificationReport.
    ///
    /// [Learn more about accessing detailed verification results.](https://stripe.com/docs/identity/verification-sessions#results).
    pub last_verification_report: Option<Expandable<IdentityVerificationReport>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// A set of options for the session’s verification checks.
    pub options: Option<GelatoVerificationSessionOptions>,

    /// Details provided about the user being verified.
    ///
    /// These details may be shown to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_details: Option<GelatoProvidedDetails>,

    /// Redaction status of this VerificationSession.
    ///
    /// If the VerificationSession is not redacted, this field will be null.
    pub redaction: Option<VerificationSessionRedaction>,

    /// Customer ID.
    pub related_customer: Option<String>,

    /// Status of this VerificationSession.
    ///
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub status: IdentityVerificationSessionStatus,

    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    pub type_: IdentityVerificationSessionType,

    /// The short-lived URL that you use to redirect a user to Stripe to submit their identity information.
    ///
    /// This URL expires after 48 hours and can only be used once.
    /// Don’t store it, log it, send it in emails or expose it to anyone other than the user.
    /// Refer to our docs on [verifying identity documents](https://stripe.com/docs/identity/verify-identity-documents?platform=web&type=redirect) to learn how to redirect users to Stripe.
    pub url: Option<String>,

    /// The configuration token of a verification flow from the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_flow: Option<String>,

    /// The user’s verified data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_outputs: Option<GelatoVerifiedOutputs>,
}

impl Object for IdentityVerificationSession {
    type Id = IdentityVerificationSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "identity.verification_session"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoProvidedDetails {

    /// Email of user being verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone number of user being verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoSessionLastError {

    /// A short machine-readable string giving the reason for the verification or user-session failure.
    pub code: Option<GelatoSessionLastErrorCode>,

    /// A message that explains the reason for verification or user-session failure.
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoVerificationSessionOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<GelatoSessionDocumentOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<GelatoSessionEmailOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<GelatoSessionIdNumberOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<GelatoSessionPhoneOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoSessionDocumentOptions {

    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<GelatoSessionDocumentOptionsAllowedTypes>>,

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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoSessionEmailOptions {

    /// Request one time password verification of `provided_details.email`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_verification: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoSessionIdNumberOptions {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoSessionPhoneOptions {

    /// Request one time password verification of `provided_details.phone`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_verification: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoVerifiedOutputs {

    /// The user's verified address.
    pub address: Option<Address>,

    /// The user’s verified date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<GelatoDataVerifiedOutputsDate>,

    /// The user's verified email address.
    pub email: Option<String>,

    /// The user's verified first name.
    pub first_name: Option<String>,

    /// The user's verified id number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    /// The user's verified id number type.
    pub id_number_type: Option<GelatoVerifiedOutputsIdNumberType>,

    /// The user's verified last name.
    pub last_name: Option<String>,

    /// The user's verified phone number.
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoDataVerifiedOutputsDate {

    /// Numerical day between 1 and 31.
    pub day: Option<i64>,

    /// Numerical month between 1 and 12.
    pub month: Option<i64>,

    /// The four-digit year.
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VerificationSessionRedaction {

    /// Indicates whether this object and its related objects have been redacted or not.
    pub status: VerificationSessionRedactionStatus,
}

/// An enum representing the possible values of an `GelatoSessionDocumentOptions`'s `allowed_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSessionDocumentOptionsAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl GelatoSessionDocumentOptionsAllowedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoSessionDocumentOptionsAllowedTypes::DrivingLicense => "driving_license",
            GelatoSessionDocumentOptionsAllowedTypes::IdCard => "id_card",
            GelatoSessionDocumentOptionsAllowedTypes::Passport => "passport",
        }
    }
}

impl AsRef<str> for GelatoSessionDocumentOptionsAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSessionDocumentOptionsAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for GelatoSessionDocumentOptionsAllowedTypes {
    fn default() -> Self {
        Self::DrivingLicense
    }
}

/// An enum representing the possible values of an `GelatoSessionLastError`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSessionLastErrorCode {
    Abandoned,
    ConsentDeclined,
    CountryNotSupported,
    DeviceNotSupported,
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
    EmailUnverifiedOther,
    EmailVerificationDeclined,
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
    PhoneUnverifiedOther,
    PhoneVerificationDeclined,
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
    UnderSupportedAge,
}

impl GelatoSessionLastErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoSessionLastErrorCode::Abandoned => "abandoned",
            GelatoSessionLastErrorCode::ConsentDeclined => "consent_declined",
            GelatoSessionLastErrorCode::CountryNotSupported => "country_not_supported",
            GelatoSessionLastErrorCode::DeviceNotSupported => "device_not_supported",
            GelatoSessionLastErrorCode::DocumentExpired => "document_expired",
            GelatoSessionLastErrorCode::DocumentTypeNotSupported => "document_type_not_supported",
            GelatoSessionLastErrorCode::DocumentUnverifiedOther => "document_unverified_other",
            GelatoSessionLastErrorCode::EmailUnverifiedOther => "email_unverified_other",
            GelatoSessionLastErrorCode::EmailVerificationDeclined => "email_verification_declined",
            GelatoSessionLastErrorCode::IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            GelatoSessionLastErrorCode::IdNumberMismatch => "id_number_mismatch",
            GelatoSessionLastErrorCode::IdNumberUnverifiedOther => "id_number_unverified_other",
            GelatoSessionLastErrorCode::PhoneUnverifiedOther => "phone_unverified_other",
            GelatoSessionLastErrorCode::PhoneVerificationDeclined => "phone_verification_declined",
            GelatoSessionLastErrorCode::SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            GelatoSessionLastErrorCode::SelfieFaceMismatch => "selfie_face_mismatch",
            GelatoSessionLastErrorCode::SelfieManipulated => "selfie_manipulated",
            GelatoSessionLastErrorCode::SelfieUnverifiedOther => "selfie_unverified_other",
            GelatoSessionLastErrorCode::UnderSupportedAge => "under_supported_age",
        }
    }
}

impl AsRef<str> for GelatoSessionLastErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSessionLastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for GelatoSessionLastErrorCode {
    fn default() -> Self {
        Self::Abandoned
    }
}

/// An enum representing the possible values of an `GelatoVerifiedOutputs`'s `id_number_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoVerifiedOutputsIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

impl GelatoVerifiedOutputsIdNumberType {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoVerifiedOutputsIdNumberType::BrCpf => "br_cpf",
            GelatoVerifiedOutputsIdNumberType::SgNric => "sg_nric",
            GelatoVerifiedOutputsIdNumberType::UsSsn => "us_ssn",
        }
    }
}

impl AsRef<str> for GelatoVerifiedOutputsIdNumberType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for GelatoVerifiedOutputsIdNumberType {
    fn default() -> Self {
        Self::BrCpf
    }
}

/// An enum representing the possible values of an `IdentityVerificationSession`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationSessionStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
}

impl IdentityVerificationSessionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IdentityVerificationSessionStatus::Canceled => "canceled",
            IdentityVerificationSessionStatus::Processing => "processing",
            IdentityVerificationSessionStatus::RequiresInput => "requires_input",
            IdentityVerificationSessionStatus::Verified => "verified",
        }
    }
}

impl AsRef<str> for IdentityVerificationSessionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdentityVerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IdentityVerificationSessionStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

/// An enum representing the possible values of an `IdentityVerificationSession`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationSessionType {
    Document,
    IdNumber,
    VerificationFlow,
}

impl IdentityVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        match self {
            IdentityVerificationSessionType::Document => "document",
            IdentityVerificationSessionType::IdNumber => "id_number",
            IdentityVerificationSessionType::VerificationFlow => "verification_flow",
        }
    }
}

impl AsRef<str> for IdentityVerificationSessionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IdentityVerificationSessionType {
    fn default() -> Self {
        Self::Document
    }
}

/// An enum representing the possible values of an `VerificationSessionRedaction`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationSessionRedactionStatus {
    Processing,
    Redacted,
}

impl VerificationSessionRedactionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            VerificationSessionRedactionStatus::Processing => "processing",
            VerificationSessionRedactionStatus::Redacted => "redacted",
        }
    }
}

impl AsRef<str> for VerificationSessionRedactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationSessionRedactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for VerificationSessionRedactionStatus {
    fn default() -> Self {
        Self::Processing
    }
}
