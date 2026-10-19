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
    pub reason: stripe_shared::IssuingDisputeEvidenceReason,
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
    reason: Option<stripe_shared::IssuingDisputeEvidenceReason>,
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
    use miniserde::{Deserialize, Result, make_place};
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
                canceled: Some(None),
                duplicate: Some(None),
                fraudulent: Some(None),
                merchandise_not_as_described: Some(None),
                no_valid_authorization: Some(None),
                not_received: Some(None),
                other: Some(None),
                reason: None,
                service_not_as_described: Some(None),
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
                self.reason.take(),
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

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
pub type IssuingDisputeEvidenceReason = stripe_shared::IssuingDisputeEvidenceReason;
