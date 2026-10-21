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
    document: Option<stripe_misc::GelatoDocumentReport>,
    email: Option<stripe_misc::GelatoEmailReport>,
    id: Option<stripe_misc::IdentityVerificationReportId>,
    id_number: Option<stripe_misc::GelatoIdNumberReport>,
    livemode: Option<bool>,
    options: Option<stripe_misc::GelatoVerificationReportOptions>,
    phone: Option<stripe_misc::GelatoPhoneReport>,
    selfie: Option<stripe_misc::GelatoSelfieReport>,
    type_: Option<IdentityVerificationReportType>,
    verification_flow: Option<Option<String>>,
    verification_session: Option<Option<String>>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: IdentityVerificationReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IdentityVerificationReportBuilder {
        type Out = IdentityVerificationReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "client_reference_id" => Deserialize::begin(&mut self.client_reference_id),
                "created" => Deserialize::begin(&mut self.created),
                "document" => stripe_types::miniserde_helpers::begin_nullable(&mut self.document),
                "email" => stripe_types::miniserde_helpers::begin_nullable(&mut self.email),
                "id" => Deserialize::begin(&mut self.id),
                "id_number" => stripe_types::miniserde_helpers::begin_nullable(&mut self.id_number),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "options" => stripe_types::miniserde_helpers::begin_nullable(&mut self.options),
                "phone" => stripe_types::miniserde_helpers::begin_nullable(&mut self.phone),
                "selfie" => stripe_types::miniserde_helpers::begin_nullable(&mut self.selfie),
                "type" => Deserialize::begin(&mut self.type_),
                "verification_flow" => Deserialize::begin(&mut self.verification_flow),
                "verification_session" => Deserialize::begin(&mut self.verification_session),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                client_reference_id: Some(None),
                created: None,
                document: None,
                email: None,
                id: None,
                id_number: None,
                livemode: None,
                options: None,
                phone: None,
                selfie: None,
                type_: None,
                verification_flow: Some(None),
                verification_session: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    client_reference_id: Some(client_reference_id),
                    created: Some(created),
                    document,
                    email,
                    id: Some(id),
                    id_number,
                    livemode: Some(livemode),
                    options,
                    phone,
                    selfie,
                    type_: Some(type_),
                    verification_flow: Some(verification_flow),
                    verification_session: Some(verification_session),
                } => Some(Self::Out {
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
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for IdentityVerificationReport {
        type Builder = IdentityVerificationReportBuilder;
    }

    impl FromValueOpt for IdentityVerificationReport {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
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
impl miniserde::Deserialize for IdentityVerificationReportType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IdentityVerificationReportType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdentityVerificationReportType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IdentityVerificationReportType);
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
