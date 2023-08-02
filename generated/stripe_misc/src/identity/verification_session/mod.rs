/// A VerificationSession guides you through the process of collecting and verifying the identities
/// of your users.
///
/// It contains details about the type of verification, such as what [verification check](/docs/identity/verification-checks) to perform.
/// Only create one VerificationSession for each verification in your system.  A VerificationSession transitions through [multiple statuses](/docs/identity/how-sessions-work) throughout its lifetime as it progresses through the verification flow.
/// The VerificationSession contains the user's verified data after verification checks are complete.  Related guide: [The Verification Sessions API](https://stripe.com/docs/identity/verification-sessions).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VerificationSession {
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
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::identity::verification_session::IdentityVerificationSessionId,
    /// If present, this property tells you the last error encountered when processing the verification.
    pub last_error: Option<stripe_misc::last_error::LastError>,
    /// ID of the most recent VerificationReport.
    ///
    /// [Learn more about accessing detailed verification results.](https://stripe.com/docs/identity/verification-sessions#results).
    pub last_verification_report: Option<
        stripe_types::Expandable<stripe_misc::identity::verification_report::VerificationReport>,
    >,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: VerificationSessionObject,
    /// A set of options for the session’s verification checks.
    pub options: Option<stripe_misc::options::Options>,
    /// Redaction status of this VerificationSession.
    ///
    /// If the VerificationSession is not redacted, this field will be null.
    pub redaction: Option<stripe_misc::redaction::Redaction>,
    /// Status of this VerificationSession.
    ///
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub status: VerificationSessionStatus,
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    pub type_: Option<VerificationSessionType>,
    /// The short-lived URL that you use to redirect a user to Stripe to submit their identity information.
    ///
    /// This URL expires after 48 hours and can only be used once.
    /// Don’t store it, log it, send it in emails or expose it to anyone other than the user.
    /// Refer to our docs on [verifying identity documents](https://stripe.com/docs/identity/verify-identity-documents?platform=web&type=redirect) to learn how to redirect users to Stripe.
    pub url: Option<String>,
    /// The user’s verified data.
    pub verified_outputs: Option<stripe_misc::verified_outputs::VerifiedOutputs>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationSessionObject {
    IdentityVerificationSession,
}

impl VerificationSessionObject {
    pub fn as_str(self) -> &'static str {
        use VerificationSessionObject::*;
        match self {
            IdentityVerificationSession => "identity.verification_session",
        }
    }
}

impl std::str::FromStr for VerificationSessionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use VerificationSessionObject::*;
        match s {
            "identity.verification_session" => Ok(IdentityVerificationSession),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationSessionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationSessionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationSessionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationSessionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for VerificationSessionObject"))
    }
}
/// Status of this VerificationSession.
///
/// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationSessionStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
}

impl VerificationSessionStatus {
    pub fn as_str(self) -> &'static str {
        use VerificationSessionStatus::*;
        match self {
            Canceled => "canceled",
            Processing => "processing",
            RequiresInput => "requires_input",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for VerificationSessionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use VerificationSessionStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "processing" => Ok(Processing),
            "requires_input" => Ok(RequiresInput),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationSessionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationSessionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationSessionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for VerificationSessionStatus"))
    }
}
/// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationSessionType {
    Document,
    IdNumber,
}

impl VerificationSessionType {
    pub fn as_str(self) -> &'static str {
        use VerificationSessionType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for VerificationSessionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use VerificationSessionType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationSessionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationSessionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationSessionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for VerificationSessionType"))
    }
}
impl stripe_types::Object for VerificationSession {
    type Id = stripe_misc::identity::verification_session::IdentityVerificationSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IdentityVerificationSessionId);
pub mod requests;
