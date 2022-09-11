// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_core::{
    ids::IdentityVerificationReportId,
    params::{Object, Timestamp},
};
use serde::{Deserialize, Serialize};

use crate::resources::{
    GelatoDocumentReport, GelatoIdNumberReport, GelatoVerificationReportOptions,
};

/// The resource representing a Stripe "GelatoVerificationReport".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
impl std::default::Default for GelatoSelfieReportErrorCode {
    fn default() -> Self {
        Self::SelfieDocumentMissingPhoto
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
impl std::default::Default for GelatoSelfieReportStatus {
    fn default() -> Self {
        Self::Unverified
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
impl std::default::Default for IdentityVerificationReportType {
    fn default() -> Self {
        Self::Document
    }
}
