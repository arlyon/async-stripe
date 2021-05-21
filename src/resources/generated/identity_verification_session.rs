// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{IdentityVerificationSessionId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Address, IdentityVerificationReport};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "GelatoVerificationSession".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityVerificationSession {
    /// Unique identifier for the object.
    pub id: IdentityVerificationSessionId,

    /// This string value can be passed to stripe.js to embed a verification flow directly into your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Hash of details on the last error encountered in the verification process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<GelatoSessionLastError>,

    /// Link to the most recent completed VerificationReport for this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_verification_report: Option<Expandable<IdentityVerificationReport>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    pub options: GelatoVerificationSessionOptions,

    /// Redaction status of this VerificationSession.
    ///
    /// If the VerificationSession is not redacted, this field will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction: Option<VerificationSessionRedaction>,

    /// Status of this VerificationSession.
    ///
    /// Read more about each [VerificationSession status](https://stripe.com/docs/identity/how-sessions-work).
    pub status: IdentityVerificationSessionStatus,

    /// Type of report requested.
    #[serde(rename = "type")]
    pub type_: IdentityVerificationSessionType,

    /// Link to the Stripe-hosted identity verification portal that you can send a user to for verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Hash of verified data about this person that results from a successful verification report.
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoSessionLastError {

    /// A short machine-readable string giving the reason for the verification or user-session failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<GelatoSessionLastErrorCode>,

    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoVerificationSessionOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<GelatoSessionDocumentOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<GelatoSessionIdNumberOptions>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoSessionDocumentOptions {

    /// Restrict the list of allowed document type to these types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<GelatoSessionDocumentOptionsAllowedTypes>>,

    /// Require that the user provide an id number which will be verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,

    /// Require that the user capture documents live with their webcam or phone camera.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,

    /// Require that the user provide a selfie to compare against the document photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoSessionIdNumberOptions {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoVerifiedOutputs {

    /// Verified address of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Verified date of birth of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<GelatoDataVerifiedOutputsDate>,

    /// Verified first name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Verified national id number of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    /// Country / type of verified national id number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_type: Option<GelatoVerifiedOutputsIdNumberType>,

    /// Verified last name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoDataVerifiedOutputsDate {

    /// Numerical day between 1 and 31.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,

    /// Numerical month between 1 and 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,

    /// The four-digit year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
