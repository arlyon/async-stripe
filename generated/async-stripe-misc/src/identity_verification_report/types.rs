/// A VerificationReport is the result of an attempt to collect and verify data from a user.
/// The collection of verification checks performed is determined from the `type` and `options`
/// parameters used. You can find the result of each verification check performed in the
/// appropriate sub-resource: `document`, `id_number`, `selfie`.
///
/// Each VerificationReport contains a copy of any data collected by the user as well as
/// reference IDs which can be used to access collected images through the [FileUpload](https://docs.stripe.com/api/files).
/// API. To configure and create VerificationReports, use the
/// [VerificationSession](https://docs.stripe.com/api/identity/verification_sessions) API.
///
/// Related guide: [Accessing verification results](https://docs.stripe.com/identity/verification-sessions#results).
///
/// For more details see <<https://stripe.com/docs/api/identity/verification_reports/object>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IdentityVerificationReport {
    /// A string to reference this user.
    /// This can be a customer ID, a session ID, or similar, and can be used to reconcile this verification with your internal systems.
    pub client_reference_id: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub document: Option<stripe_misc::GelatoDocumentReport>,
    pub email: Option<stripe_misc::GelatoEmailReport>,
    /// Unique identifier for the object.
    pub id: stripe_misc::IdentityVerificationReportId,
    pub id_number: Option<stripe_misc::GelatoIdNumberReport>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub options: Option<stripe_misc::GelatoVerificationReportOptions>,
    pub phone: Option<stripe_misc::GelatoPhoneReport>,
    pub selfie: Option<stripe_misc::GelatoSelfieReport>,
    /// Type of report.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: IdentityVerificationReportType,
    /// The configuration token of a verification flow from the dashboard.
    pub verification_flow: Option<String>,
    /// ID of the VerificationSession that created this report.
    pub verification_session: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IdentityVerificationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IdentityVerificationReport").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IdentityVerificationReportBuilder {
    client_reference_id: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    document: Option<Option<stripe_misc::GelatoDocumentReport>>,
    email: Option<Option<stripe_misc::GelatoEmailReport>>,
    id: Option<stripe_misc::IdentityVerificationReportId>,
    id_number: Option<Option<stripe_misc::GelatoIdNumberReport>>,
    livemode: Option<bool>,
    options: Option<Option<stripe_misc::GelatoVerificationReportOptions>>,
    phone: Option<Option<stripe_misc::GelatoPhoneReport>>,
    selfie: Option<Option<stripe_misc::GelatoSelfieReport>>,
    type_: Option<IdentityVerificationReportType>,
    verification_flow: Option<Option<String>>,
    verification_session: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for IdentityVerificationReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IdentityVerificationReport>,
        builder: IdentityVerificationReportBuilder,
    }

    impl Visitor for Place<IdentityVerificationReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IdentityVerificationReportBuilder {
                    client_reference_id: Deserialize::default(),
                    created: Deserialize::default(),
                    document: Deserialize::default(),
                    email: Deserialize::default(),
                    id: Deserialize::default(),
                    id_number: Deserialize::default(),
                    livemode: Deserialize::default(),
                    options: Deserialize::default(),
                    phone: Deserialize::default(),
                    selfie: Deserialize::default(),
                    type_: Deserialize::default(),
                    verification_flow: Deserialize::default(),
                    verification_session: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "client_reference_id" => Deserialize::begin(&mut self.builder.client_reference_id),
                "created" => Deserialize::begin(&mut self.builder.created),
                "document" => Deserialize::begin(&mut self.builder.document),
                "email" => Deserialize::begin(&mut self.builder.email),
                "id" => Deserialize::begin(&mut self.builder.id),
                "id_number" => Deserialize::begin(&mut self.builder.id_number),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "options" => Deserialize::begin(&mut self.builder.options),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                "selfie" => Deserialize::begin(&mut self.builder.selfie),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "verification_flow" => Deserialize::begin(&mut self.builder.verification_flow),
                "verification_session" => {
                    Deserialize::begin(&mut self.builder.verification_session)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(client_reference_id),
                Some(created),
                Some(document),
                Some(email),
                Some(id),
                Some(id_number),
                Some(livemode),
                Some(options),
                Some(phone),
                Some(selfie),
                Some(type_),
                Some(verification_flow),
                Some(verification_session),
            ) = (
                self.builder.client_reference_id.take(),
                self.builder.created,
                self.builder.document.take(),
                self.builder.email.take(),
                self.builder.id.take(),
                self.builder.id_number.take(),
                self.builder.livemode,
                self.builder.options.take(),
                self.builder.phone.take(),
                self.builder.selfie.take(),
                self.builder.type_.take(),
                self.builder.verification_flow.take(),
                self.builder.verification_session.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IdentityVerificationReport {
                client_reference_id,
                created,
                document,
                email,
                id,
                id_number,
                livemode,
                options,
                phone,
                selfie,
                type_,
                verification_flow,
                verification_session,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IdentityVerificationReport {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IdentityVerificationReport", 14)?;
        s.serialize_field("client_reference_id", &self.client_reference_id)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("document", &self.document)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("id_number", &self.id_number)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("options", &self.options)?;
        s.serialize_field("phone", &self.phone)?;
        s.serialize_field("selfie", &self.selfie)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("verification_flow", &self.verification_flow)?;
        s.serialize_field("verification_session", &self.verification_session)?;

        s.serialize_field("object", "identity.verification_report")?;
        s.end()
    }
}
/// Type of report.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IdentityVerificationReportType {
    Document,
    IdNumber,
    VerificationFlow,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IdentityVerificationReportType {
    pub fn as_str(&self) -> &str {
        use IdentityVerificationReportType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
            VerificationFlow => "verification_flow",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IdentityVerificationReportType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdentityVerificationReportType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            "verification_flow" => Ok(VerificationFlow),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IdentityVerificationReportType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IdentityVerificationReportType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IdentityVerificationReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IdentityVerificationReportType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IdentityVerificationReportType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdentityVerificationReportType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IdentityVerificationReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for IdentityVerificationReport {
    type Id = stripe_misc::IdentityVerificationReportId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(IdentityVerificationReportId);
