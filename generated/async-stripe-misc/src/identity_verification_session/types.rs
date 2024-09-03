/// A VerificationSession guides you through the process of collecting and verifying the identities
/// of your users. It contains details about the type of verification, such as what [verification
/// check](/docs/identity/verification-checks) to perform. Only create one VerificationSession for
/// each verification in your system.
///
/// A VerificationSession transitions through [multiple
/// statuses](/docs/identity/how-sessions-work) throughout its lifetime as it progresses through
/// the verification flow. The VerificationSession contains the user's verified data after
/// verification checks are complete.
///
/// Related guide: [The Verification Sessions API](https://stripe.com/docs/identity/verification-sessions).
///
/// For more details see <<https://stripe.com/docs/api/identity/verification_sessions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IdentityVerificationSession {
    /// A string to reference this user.
    /// This can be a customer ID, a session ID, or similar, and can be used to reconcile this verification with your internal systems.
    pub client_reference_id: Option<String>,
    /// The short-lived client secret used by Stripe.js to [show a verification modal](https://stripe.com/docs/js/identity/modal) inside your app.
    /// This client secret expires after 24 hours and can only be used once.
    /// Don’t store it, log it, embed it in a URL, or expose it to anyone other than the user.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    /// Refer to our docs on [passing the client secret to the frontend](https://stripe.com/docs/identity/verification-sessions#client-secret) to learn more.
    pub client_secret: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::IdentityVerificationSessionId,
    /// If present, this property tells you the last error encountered when processing the verification.
    pub last_error: Option<stripe_misc::GelatoSessionLastError>,
    /// ID of the most recent VerificationReport.
    /// [Learn more about accessing detailed verification results.](https://stripe.com/docs/identity/verification-sessions#results).
    pub last_verification_report:
        Option<stripe_types::Expandable<stripe_misc::IdentityVerificationReport>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// A set of options for the session’s verification checks.
    pub options: Option<stripe_misc::GelatoVerificationSessionOptions>,
    /// Details provided about the user being verified. These details may be shown to the user.
    pub provided_details: Option<stripe_misc::GelatoProvidedDetails>,
    /// Redaction status of this VerificationSession.
    /// If the VerificationSession is not redacted, this field will be null.
    pub redaction: Option<stripe_misc::VerificationSessionRedaction>,
    /// Status of this VerificationSession.
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub status: stripe_misc::IdentityVerificationSessionStatus,
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: IdentityVerificationSessionType,
    /// The short-lived URL that you use to redirect a user to Stripe to submit their identity information.
    /// This URL expires after 48 hours and can only be used once.
    /// Don’t store it, log it, send it in emails or expose it to anyone other than the user.
    /// Refer to our docs on [verifying identity documents](https://stripe.com/docs/identity/verify-identity-documents?platform=web&type=redirect) to learn how to redirect users to Stripe.
    pub url: Option<String>,
    /// The configuration token of a Verification Flow from the dashboard.
    pub verification_flow: Option<String>,
    /// The user’s verified data.
    pub verified_outputs: Option<stripe_misc::GelatoVerifiedOutputs>,
}
#[doc(hidden)]
pub struct IdentityVerificationSessionBuilder {
    client_reference_id: Option<Option<String>>,
    client_secret: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_misc::IdentityVerificationSessionId>,
    last_error: Option<Option<stripe_misc::GelatoSessionLastError>>,
    last_verification_report:
        Option<Option<stripe_types::Expandable<stripe_misc::IdentityVerificationReport>>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    options: Option<Option<stripe_misc::GelatoVerificationSessionOptions>>,
    provided_details: Option<Option<stripe_misc::GelatoProvidedDetails>>,
    redaction: Option<Option<stripe_misc::VerificationSessionRedaction>>,
    status: Option<stripe_misc::IdentityVerificationSessionStatus>,
    type_: Option<IdentityVerificationSessionType>,
    url: Option<Option<String>>,
    verification_flow: Option<Option<String>>,
    verified_outputs: Option<Option<stripe_misc::GelatoVerifiedOutputs>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IdentityVerificationSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IdentityVerificationSession>,
        builder: IdentityVerificationSessionBuilder,
    }

    impl Visitor for Place<IdentityVerificationSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IdentityVerificationSessionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IdentityVerificationSessionBuilder {
        type Out = IdentityVerificationSession;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "client_reference_id" => Deserialize::begin(&mut self.client_reference_id),
                "client_secret" => Deserialize::begin(&mut self.client_secret),
                "created" => Deserialize::begin(&mut self.created),
                "id" => Deserialize::begin(&mut self.id),
                "last_error" => Deserialize::begin(&mut self.last_error),
                "last_verification_report" => {
                    Deserialize::begin(&mut self.last_verification_report)
                }
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "options" => Deserialize::begin(&mut self.options),
                "provided_details" => Deserialize::begin(&mut self.provided_details),
                "redaction" => Deserialize::begin(&mut self.redaction),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),
                "url" => Deserialize::begin(&mut self.url),
                "verification_flow" => Deserialize::begin(&mut self.verification_flow),
                "verified_outputs" => Deserialize::begin(&mut self.verified_outputs),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                client_reference_id: Deserialize::default(),
                client_secret: Deserialize::default(),
                created: Deserialize::default(),
                id: Deserialize::default(),
                last_error: Deserialize::default(),
                last_verification_report: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                options: Deserialize::default(),
                provided_details: Deserialize::default(),
                redaction: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
                url: Deserialize::default(),
                verification_flow: Deserialize::default(),
                verified_outputs: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(client_reference_id),
                Some(client_secret),
                Some(created),
                Some(id),
                Some(last_error),
                Some(last_verification_report),
                Some(livemode),
                Some(metadata),
                Some(options),
                Some(provided_details),
                Some(redaction),
                Some(status),
                Some(type_),
                Some(url),
                Some(verification_flow),
                Some(verified_outputs),
            ) = (
                self.client_reference_id.take(),
                self.client_secret.take(),
                self.created,
                self.id.take(),
                self.last_error.take(),
                self.last_verification_report.take(),
                self.livemode,
                self.metadata.take(),
                self.options.take(),
                self.provided_details.take(),
                self.redaction,
                self.status,
                self.type_,
                self.url.take(),
                self.verification_flow.take(),
                self.verified_outputs.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                client_reference_id,
                client_secret,
                created,
                id,
                last_error,
                last_verification_report,
                livemode,
                metadata,
                options,
                provided_details,
                redaction,
                status,
                type_,
                url,
                verification_flow,
                verified_outputs,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IdentityVerificationSession {
        type Builder = IdentityVerificationSessionBuilder;
    }

    impl FromValueOpt for IdentityVerificationSession {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IdentityVerificationSessionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "client_reference_id" => b.client_reference_id = FromValueOpt::from_value(v),
                    "client_secret" => b.client_secret = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "last_error" => b.last_error = FromValueOpt::from_value(v),
                    "last_verification_report" => {
                        b.last_verification_report = FromValueOpt::from_value(v)
                    }
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "options" => b.options = FromValueOpt::from_value(v),
                    "provided_details" => b.provided_details = FromValueOpt::from_value(v),
                    "redaction" => b.redaction = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    "verification_flow" => b.verification_flow = FromValueOpt::from_value(v),
                    "verified_outputs" => b.verified_outputs = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IdentityVerificationSession {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IdentityVerificationSession", 17)?;
        s.serialize_field("client_reference_id", &self.client_reference_id)?;
        s.serialize_field("client_secret", &self.client_secret)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("last_error", &self.last_error)?;
        s.serialize_field("last_verification_report", &self.last_verification_report)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("options", &self.options)?;
        s.serialize_field("provided_details", &self.provided_details)?;
        s.serialize_field("redaction", &self.redaction)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("url", &self.url)?;
        s.serialize_field("verification_flow", &self.verification_flow)?;
        s.serialize_field("verified_outputs", &self.verified_outputs)?;

        s.serialize_field("object", "identity.verification_session")?;
        s.end()
    }
}
/// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IdentityVerificationSessionType {
    Document,
    IdNumber,
    VerificationFlow,
}
impl IdentityVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        use IdentityVerificationSessionType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
            VerificationFlow => "verification_flow",
        }
    }
}

impl std::str::FromStr for IdentityVerificationSessionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdentityVerificationSessionType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            "verification_flow" => Ok(VerificationFlow),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IdentityVerificationSessionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IdentityVerificationSessionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IdentityVerificationSessionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IdentityVerificationSessionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IdentityVerificationSessionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IdentityVerificationSessionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IdentityVerificationSessionType")
        })
    }
}
impl stripe_types::Object for IdentityVerificationSession {
    type Id = stripe_misc::IdentityVerificationSessionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(IdentityVerificationSessionId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IdentityVerificationSessionStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
}
impl IdentityVerificationSessionStatus {
    pub fn as_str(self) -> &'static str {
        use IdentityVerificationSessionStatus::*;
        match self {
            Canceled => "canceled",
            Processing => "processing",
            RequiresInput => "requires_input",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for IdentityVerificationSessionStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdentityVerificationSessionStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "processing" => Ok(Processing),
            "requires_input" => Ok(RequiresInput),
            "verified" => Ok(Verified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IdentityVerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IdentityVerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IdentityVerificationSessionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IdentityVerificationSessionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IdentityVerificationSessionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IdentityVerificationSessionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IdentityVerificationSessionStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IdentityVerificationSessionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IdentityVerificationSessionStatus")
        })
    }
}
