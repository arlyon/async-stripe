/// A VerificationReport is the result of an attempt to collect and verify data from a user.
/// The collection of verification checks performed is determined from the `type` and `options`
/// parameters used. You can find the result of each verification check performed in the
/// appropriate sub-resource: `document`, `id_number`, `selfie`.
///
/// Each VerificationReport contains a copy of any data collected by the user as well as
/// reference IDs which can be used to access collected images through the [FileUpload](https://stripe.com/docs/api/files).
/// API. To configure and create VerificationReports, use the
/// [VerificationSession](https://stripe.com/docs/api/identity/verification_sessions) API.
///
/// Related guides: [Accessing verification results](https://stripe.com/docs/identity/verification-sessions#results).
///
/// For more details see <<https://stripe.com/docs/api/identity/verification_reports/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IdentityVerificationReport {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_misc::GelatoDocumentReport>,
    /// Unique identifier for the object.
    pub id: stripe_misc::IdentityVerificationReportId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<stripe_misc::GelatoIdNumberReport>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<stripe_misc::GelatoVerificationReportOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<stripe_misc::GelatoSelfieReport>,
    /// Type of report.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_misc::IdentityVerificationReportType>,
    /// ID of the VerificationSession that created this report.
    pub verification_session: Option<String>,
}
impl stripe_types::Object for IdentityVerificationReport {
    type Id = stripe_misc::IdentityVerificationReportId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IdentityVerificationReportId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IdentityVerificationReportType {
    Document,
    IdNumber,
}
impl IdentityVerificationReportType {
    pub fn as_str(self) -> &'static str {
        use IdentityVerificationReportType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for IdentityVerificationReportType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdentityVerificationReportType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IdentityVerificationReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdentityVerificationReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IdentityVerificationReportType")
        })
    }
}
