/// A VerificationReport is the result of an attempt to collect and verify data from a user.
/// The collection of verification checks performed is determined from the `type` and `options`
/// parameters used.
///
/// You can find the result of each verification check performed in the appropriate sub-resource: `document`, `id_number`, `selfie`.  Each VerificationReport contains a copy of any data collected by the user as well as reference IDs which can be used to access collected images through the [FileUpload](https://stripe.com/docs/api/files) API.
/// To configure and create VerificationReports, use the [VerificationSession](https://stripe.com/docs/api/identity/verification_sessions) API.  Related guides: [Accessing verification results](https://stripe.com/docs/identity/verification-sessions#results).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct VerificationReport {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<crate::identity::verification_report::document::Document>,
    /// Unique identifier for the object.
    pub id: crate::identity::verification_report::IdentityVerificationReportId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<crate::identity::verification_report::id_number::IdNumber>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: VerificationReportObject,
    pub options: crate::identity::verification_report::options::Options,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<crate::identity::verification_report::selfie::Selfie>,
    /// Type of report.
    #[serde(rename = "type")]
    pub type_: VerificationReportType,
    /// ID of the VerificationSession that created this report.
    pub verification_session: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for VerificationReport {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum VerificationReportObject {
    #[serde(rename = "identity.verification_report")]
    IdentityVerificationReport,
}

impl VerificationReportObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IdentityVerificationReport => "identity.verification_report",
        }
    }
}

impl AsRef<str> for VerificationReportObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationReportObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Type of report.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum VerificationReportType {
    Document,
    IdNumber,
}

impl VerificationReportType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::IdNumber => "id_number",
        }
    }
}

impl AsRef<str> for VerificationReportType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for VerificationReport {
    type Id = crate::identity::verification_report::IdentityVerificationReportId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(IdentityVerificationReportId);
pub mod document;
pub mod requests;
pub use document::Document;
pub mod id_number;
pub use id_number::IdNumber;
pub mod selfie;
pub use selfie::Selfie;
pub mod options;
pub use options::Options;
