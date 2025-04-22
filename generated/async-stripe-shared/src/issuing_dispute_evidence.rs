#[derive(Clone, Debug)]
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
                builder: IssuingDisputeEvidenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingDisputeEvidenceBuilder {
        type Out = IssuingDisputeEvidence;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled" => Deserialize::begin(&mut self.canceled),
                "duplicate" => Deserialize::begin(&mut self.duplicate),
                "fraudulent" => Deserialize::begin(&mut self.fraudulent),
                "merchandise_not_as_described" => {
                    Deserialize::begin(&mut self.merchandise_not_as_described)
                }
                "no_valid_authorization" => Deserialize::begin(&mut self.no_valid_authorization),
                "not_received" => Deserialize::begin(&mut self.not_received),
                "other" => Deserialize::begin(&mut self.other),
                "reason" => Deserialize::begin(&mut self.reason),
                "service_not_as_described" => {
                    Deserialize::begin(&mut self.service_not_as_described)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                canceled: Deserialize::default(),
                duplicate: Deserialize::default(),
                fraudulent: Deserialize::default(),
                merchandise_not_as_described: Deserialize::default(),
                no_valid_authorization: Deserialize::default(),
                not_received: Deserialize::default(),
                other: Deserialize::default(),
                reason: Deserialize::default(),
                service_not_as_described: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.canceled.take(),
                self.duplicate.take(),
                self.fraudulent.take(),
                self.merchandise_not_as_described.take(),
                self.no_valid_authorization.take(),
                self.not_received.take(),
                self.other.take(),
                self.reason,
                self.service_not_as_described.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                canceled,
                duplicate,
                fraudulent,
                merchandise_not_as_described,
                no_valid_authorization,
                not_received,
                other,
                reason,
                service_not_as_described,
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

    impl ObjectDeser for IssuingDisputeEvidence {
        type Builder = IssuingDisputeEvidenceBuilder;
    }

    impl FromValueOpt for IssuingDisputeEvidence {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingDisputeEvidenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "canceled" => b.canceled = FromValueOpt::from_value(v),
                    "duplicate" => b.duplicate = FromValueOpt::from_value(v),
                    "fraudulent" => b.fraudulent = FromValueOpt::from_value(v),
                    "merchandise_not_as_described" => {
                        b.merchandise_not_as_described = FromValueOpt::from_value(v)
                    }
                    "no_valid_authorization" => {
                        b.no_valid_authorization = FromValueOpt::from_value(v)
                    }
                    "not_received" => b.not_received = FromValueOpt::from_value(v),
                    "other" => b.other = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    "service_not_as_described" => {
                        b.service_not_as_described = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The reason for filing the dispute. Its value will match the field containing the evidence.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NoValidAuthorization,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}
impl IssuingDisputeEvidenceReason {
    pub fn as_str(self) -> &'static str {
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
        }
    }
}

impl std::str::FromStr for IssuingDisputeEvidenceReason {
    type Err = stripe_types::StripeParseError;
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
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for IssuingDisputeEvidenceReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingDisputeEvidenceReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingDisputeEvidenceReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingDisputeEvidenceReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingDisputeEvidenceReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingDisputeEvidenceReason"))
    }
}
