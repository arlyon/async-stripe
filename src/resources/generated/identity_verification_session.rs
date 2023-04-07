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

    /// Redaction status of this VerificationSession.
    ///
    /// If the VerificationSession is not redacted, this field will be null.
    pub redaction: Option<VerificationSessionRedaction>,

    /// Status of this VerificationSession.
    ///
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub status: IdentityVerificationSessionStatus,

    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    pub type_: Option<IdentityVerificationSessionType>,

    /// The short-lived URL that you use to redirect a user to Stripe to submit their identity information.
    ///
    /// This URL expires after 48 hours and can only be used once.
    /// Don’t store it, log it, send it in emails or expose it to anyone other than the user.
    /// Refer to our docs on [verifying identity documents](https://stripe.com/docs/identity/verify-identity-documents?platform=web&type=redirect) to learn how to redirect users to Stripe.
    pub url: Option<String>,

    /// The user’s verified data.
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
    pub id_number: Option<GelatoSessionIdNumberOptions>,
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
pub struct GelatoSessionIdNumberOptions {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GelatoVerifiedOutputs {

    /// The user's verified address.
    pub address: Option<Address>,

    /// The user’s verified date of birth.
    pub dob: Option<GelatoDataVerifiedOutputsDate>,

    /// The user's verified first name.
    pub first_name: Option<String>,

    /// The user's verified id number.
    pub id_number: Option<String>,

    /// The user's verified id number type.
    pub id_number_type: Option<GelatoVerifiedOutputsIdNumberType>,

    /// The user's verified last name.
    pub last_name: Option<String>,
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
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
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
            GelatoSessionLastErrorCode::IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            GelatoSessionLastErrorCode::IdNumberMismatch => "id_number_mismatch",
            GelatoSessionLastErrorCode::IdNumberUnverifiedOther => "id_number_unverified_other",
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
}

impl IdentityVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        match self {
            IdentityVerificationSessionType::Document => "document",
            IdentityVerificationSessionType::IdNumber => "id_number",
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
