// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::IdentityVerificationReportId;
use crate::params::{Object, Timestamp};
use crate::resources::Address;

/// The resource representing a Stripe "GelatoVerificationReport".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityVerificationReport {
    /// Unique identifier for the object.
    pub id: IdentityVerificationReportId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<GelatoDocumentReport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<GelatoIdNumberReport>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub options: GelatoVerificationReportOptions,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<GelatoSelfieReport>,

    /// Type of report.
    #[serde(rename = "type")]
    pub type_: IdentityVerificationReportType,

    /// ID of the VerificationSession that created this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<String>,
}

impl Object for IdentityVerificationReport {
    type Id = IdentityVerificationReportId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "identity.verification_report"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoDocumentReport {
    /// Address as it appears in the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Date of birth as it appears in the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<GelatoDataDocumentReportDateOfBirth>,

    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<GelatoDocumentReportError>,

    /// Expiration date of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<GelatoDataDocumentReportExpirationDate>,

    /// Array of [File](https://stripe.com/docs/api/files) ids containing images for this document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,

    /// First name as it appears in the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Issued date of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_date: Option<GelatoDataDocumentReportIssuedDate>,

    /// Issuing country of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<String>,

    /// Last name as it appears in the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Document ID number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Status of this `document` check.
    pub status: GelatoDocumentReportStatus,

    /// Type of the document.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<GelatoDocumentReportType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoDataDocumentReportDateOfBirth {
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
pub struct GelatoDataDocumentReportExpirationDate {
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
pub struct GelatoDataDocumentReportIssuedDate {
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
pub struct GelatoDocumentReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<GelatoDocumentReportErrorCode>,

    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoIdNumberReport {
    /// Date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<GelatoDataIdNumberReportDate>,

    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<GelatoIdNumberReportError>,

    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// ID number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    /// Type of ID number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_type: Option<GelatoIdNumberReportIdNumberType>,

    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Status of this `id_number` check.
    pub status: GelatoIdNumberReportStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoDataIdNumberReportDate {
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
pub struct GelatoIdNumberReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<GelatoIdNumberReportErrorCode>,

    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoSelfieReport {
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the identity document used in this check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,

    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<GelatoSelfieReportError>,

    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the selfie used in this check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<String>,

    /// Status of this `selfie` check.
    pub status: GelatoSelfieReportStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoSelfieReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<GelatoSelfieReportErrorCode>,

    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoVerificationReportOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<GelatoReportDocumentOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<GelatoReportIdNumberOptions>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoReportDocumentOptions {
    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<GelatoReportDocumentOptionsAllowedTypes>>,

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GelatoReportIdNumberOptions {}

/// An enum representing the possible values of an `GelatoDocumentReportError`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoDocumentReportErrorCode {
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
}

impl GelatoDocumentReportErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoDocumentReportErrorCode::DocumentExpired => "document_expired",
            GelatoDocumentReportErrorCode::DocumentTypeNotSupported => {
                "document_type_not_supported"
            }
            GelatoDocumentReportErrorCode::DocumentUnverifiedOther => "document_unverified_other",
        }
    }
}

impl AsRef<str> for GelatoDocumentReportErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoDocumentReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `GelatoDocumentReport`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoDocumentReportStatus {
    Unverified,
    Verified,
}

impl GelatoDocumentReportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoDocumentReportStatus::Unverified => "unverified",
            GelatoDocumentReportStatus::Verified => "verified",
        }
    }
}

impl AsRef<str> for GelatoDocumentReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoDocumentReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `GelatoDocumentReport`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoDocumentReportType {
    DrivingLicense,
    IdCard,
    Passport,
}

impl GelatoDocumentReportType {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoDocumentReportType::DrivingLicense => "driving_license",
            GelatoDocumentReportType::IdCard => "id_card",
            GelatoDocumentReportType::Passport => "passport",
        }
    }
}

impl AsRef<str> for GelatoDocumentReportType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoDocumentReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `GelatoIdNumberReportError`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoIdNumberReportErrorCode {
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
}

impl GelatoIdNumberReportErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoIdNumberReportErrorCode::IdNumberInsufficientDocumentData => {
                "id_number_insufficient_document_data"
            }
            GelatoIdNumberReportErrorCode::IdNumberMismatch => "id_number_mismatch",
            GelatoIdNumberReportErrorCode::IdNumberUnverifiedOther => "id_number_unverified_other",
        }
    }
}

impl AsRef<str> for GelatoIdNumberReportErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoIdNumberReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `GelatoIdNumberReport`'s `id_number_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoIdNumberReportIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

impl GelatoIdNumberReportIdNumberType {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoIdNumberReportIdNumberType::BrCpf => "br_cpf",
            GelatoIdNumberReportIdNumberType::SgNric => "sg_nric",
            GelatoIdNumberReportIdNumberType::UsSsn => "us_ssn",
        }
    }
}

impl AsRef<str> for GelatoIdNumberReportIdNumberType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoIdNumberReportIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `GelatoIdNumberReport`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoIdNumberReportStatus {
    Unverified,
    Verified,
}

impl GelatoIdNumberReportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoIdNumberReportStatus::Unverified => "unverified",
            GelatoIdNumberReportStatus::Verified => "verified",
        }
    }
}

impl AsRef<str> for GelatoIdNumberReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoIdNumberReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `GelatoReportDocumentOptions`'s `allowed_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoReportDocumentOptionsAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl GelatoReportDocumentOptionsAllowedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoReportDocumentOptionsAllowedTypes::DrivingLicense => "driving_license",
            GelatoReportDocumentOptionsAllowedTypes::IdCard => "id_card",
            GelatoReportDocumentOptionsAllowedTypes::Passport => "passport",
        }
    }
}

impl AsRef<str> for GelatoReportDocumentOptionsAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoReportDocumentOptionsAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `GelatoSelfieReportError`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSelfieReportErrorCode {
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
}

impl GelatoSelfieReportErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoSelfieReportErrorCode::SelfieDocumentMissingPhoto => {
                "selfie_document_missing_photo"
            }
            GelatoSelfieReportErrorCode::SelfieFaceMismatch => "selfie_face_mismatch",
            GelatoSelfieReportErrorCode::SelfieManipulated => "selfie_manipulated",
            GelatoSelfieReportErrorCode::SelfieUnverifiedOther => "selfie_unverified_other",
        }
    }
}

impl AsRef<str> for GelatoSelfieReportErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSelfieReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `GelatoSelfieReport`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSelfieReportStatus {
    Unverified,
    Verified,
}

impl GelatoSelfieReportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            GelatoSelfieReportStatus::Unverified => "unverified",
            GelatoSelfieReportStatus::Verified => "verified",
        }
    }
}

impl AsRef<str> for GelatoSelfieReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSelfieReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IdentityVerificationReport`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationReportType {
    Document,
    IdNumber,
}

impl IdentityVerificationReportType {
    pub fn as_str(self) -> &'static str {
        match self {
            IdentityVerificationReportType::Document => "document",
            IdentityVerificationReportType::IdNumber => "id_number",
        }
    }
}

impl AsRef<str> for IdentityVerificationReportType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
