#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeEvidence {
    pub canceled: Option<stripe_shared::IssuingDisputeCanceledEvidence>,
    pub duplicate: Option<stripe_shared::IssuingDisputeDuplicateEvidence>,
    pub fraudulent: Option<stripe_shared::IssuingDisputeFraudulentEvidence>,
    pub merchandise_not_as_described:
        Option<stripe_shared::IssuingDisputeMerchandiseNotAsDescribedEvidence>,
    pub no_valid_authorization: Option<stripe_shared::IssuingDisputeNoValidAuthorizationEvidence>,
    pub not_received: Option<stripe_shared::IssuingDisputeNotReceivedEvidence>,
    pub other: Option<stripe_shared::IssuingDisputeOtherEvidence>,
    /// The reason for filing the dispute. Its value will match the field containing the evidence.
    pub reason: IssuingDisputeEvidenceReason,
    pub service_not_as_described:
        Option<stripe_shared::IssuingDisputeServiceNotAsDescribedEvidence>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingDisputeEvidence").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingDisputeEvidenceBuilder {
    canceled: Option<Option<stripe_shared::IssuingDisputeCanceledEvidence>>,
    duplicate: Option<Option<stripe_shared::IssuingDisputeDuplicateEvidence>>,
    fraudulent: Option<Option<stripe_shared::IssuingDisputeFraudulentEvidence>>,
    merchandise_not_as_described:
        Option<Option<stripe_shared::IssuingDisputeMerchandiseNotAsDescribedEvidence>>,
    no_valid_authorization:
        Option<Option<stripe_shared::IssuingDisputeNoValidAuthorizationEvidence>>,
    not_received: Option<Option<stripe_shared::IssuingDisputeNotReceivedEvidence>>,
    other: Option<Option<stripe_shared::IssuingDisputeOtherEvidence>>,
    reason: Option<IssuingDisputeEvidenceReason>,
    service_not_as_described:
        Option<Option<stripe_shared::IssuingDisputeServiceNotAsDescribedEvidence>>,
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

    impl Deserialize for IssuingDisputeEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeEvidence>,
        builder: IssuingDisputeEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeEvidenceBuilder {
                    canceled: Deserialize::default(),
                    duplicate: Deserialize::default(),
                    fraudulent: Deserialize::default(),
                    merchandise_not_as_described: Deserialize::default(),
                    no_valid_authorization: Deserialize::default(),
                    not_received: Deserialize::default(),
                    other: Deserialize::default(),
                    reason: Deserialize::default(),
                    service_not_as_described: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled" => Deserialize::begin(&mut self.builder.canceled),
                "duplicate" => Deserialize::begin(&mut self.builder.duplicate),
                "fraudulent" => Deserialize::begin(&mut self.builder.fraudulent),
                "merchandise_not_as_described" => {
                    Deserialize::begin(&mut self.builder.merchandise_not_as_described)
                }
                "no_valid_authorization" => {
                    Deserialize::begin(&mut self.builder.no_valid_authorization)
                }
                "not_received" => Deserialize::begin(&mut self.builder.not_received),
                "other" => Deserialize::begin(&mut self.builder.other),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                "service_not_as_described" => {
                    Deserialize::begin(&mut self.builder.service_not_as_described)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(canceled),
                Some(duplicate),
                Some(fraudulent),
                Some(merchandise_not_as_described),
                Some(no_valid_authorization),
                Some(not_received),
                Some(other),
                Some(reason),
                Some(service_not_as_described),
            ) = (
                self.builder.canceled.take(),
                self.builder.duplicate.take(),
                self.builder.fraudulent.take(),
                self.builder.merchandise_not_as_described.take(),
                self.builder.no_valid_authorization.take(),
                self.builder.not_received.take(),
                self.builder.other.take(),
                self.builder.reason.take(),
                self.builder.service_not_as_described.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingDisputeEvidence {
                canceled,
                duplicate,
                fraudulent,
                merchandise_not_as_described,
                no_valid_authorization,
                not_received,
                other,
                reason,
                service_not_as_described,
            });
            Ok(())
        }
    }
};
/// The reason for filing the dispute. Its value will match the field containing the evidence.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NoValidAuthorization,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingDisputeEvidenceReason {
    pub fn as_str(&self) -> &str {
        use IssuingDisputeEvidenceReason::*;
        match self {
            Canceled => "canceled",
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            MerchandiseNotAsDescribed => "merchandise_not_as_described",
            NoValidAuthorization => "no_valid_authorization",
            NotReceived => "not_received",
            Other => "other",
            ServiceNotAsDescribed => "service_not_as_described",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingDisputeEvidenceReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeEvidenceReason::*;
        match s {
            "canceled" => Ok(Canceled),
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "merchandise_not_as_described" => Ok(MerchandiseNotAsDescribed),
            "no_valid_authorization" => Ok(NoValidAuthorization),
            "not_received" => Ok(NotReceived),
            "other" => Ok(Other),
            "service_not_as_described" => Ok(ServiceNotAsDescribed),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingDisputeEvidenceReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingDisputeEvidenceReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingDisputeEvidenceReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingDisputeEvidenceReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingDisputeEvidenceReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingDisputeEvidenceReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingDisputeEvidenceReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
