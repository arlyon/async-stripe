/// A VerificationReport is the result of an attempt to collect and verify data from a user.
/// The collection of verification checks performed is determined from the `type` and `options`
/// parameters used.
///
/// You can find the result of each verification check performed in the appropriate sub-resource: `document`, `id_number`, `selfie`.  Each VerificationReport contains a copy of any data collected by the user as well as reference IDs which can be used to access collected images through the [FileUpload](https://stripe.com/docs/api/files) API.
/// To configure and create VerificationReports, use the [VerificationSession](https://stripe.com/docs/api/identity/verification_sessions) API.  Related guides: [Accessing verification results](https://stripe.com/docs/identity/verification-sessions#results).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VerificationReport {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_misc::identity::verification_report::document::Document>,
    /// Unique identifier for the object.
    pub id: stripe_misc::identity::verification_report::IdentityVerificationReportId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<stripe_misc::identity::verification_report::id_number::IdNumber>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: VerificationReportObject,
    pub options: stripe_misc::identity::verification_report::options::Options,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<stripe_misc::identity::verification_report::selfie::Selfie>,
    /// Type of report.
    #[serde(rename = "type")]
    pub type_: VerificationReportType,
    /// ID of the VerificationSession that created this report.
    pub verification_session: Option<String>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationReportObject {
    IdentityVerificationReport,
}

impl VerificationReportObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IdentityVerificationReport => "identity.verification_report",
        }
    }
}

impl std::str::FromStr for VerificationReportObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "identity.verification_report" => Ok(Self::IdentityVerificationReport),

            _ => Err(()),
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
impl serde::Serialize for VerificationReportObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationReportObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for VerificationReportObject"))
    }
}
/// Type of report.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for VerificationReportType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "document" => Ok(Self::Document),
            "id_number" => Ok(Self::IdNumber),

            _ => Err(()),
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
impl serde::Serialize for VerificationReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for VerificationReportType"))
    }
}
impl stripe_types::Object for VerificationReport {
    type Id = stripe_misc::identity::verification_report::IdentityVerificationReportId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IdentityVerificationReportId);
pub mod document;
pub use document::Document;
pub mod id_number;
pub use id_number::IdNumber;
pub mod selfie;
pub use selfie::Selfie;
pub mod options;
pub use options::Options;
