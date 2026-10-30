#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEvidenceDetails {
    /// Date by which evidence must be submitted in order to successfully challenge dispute.
    /// Will be 0 if the customer's bank or credit card company doesn't allow a response for this particular dispute.
    pub due_by: Option<stripe_types::Timestamp>,
    pub enhanced_eligibility: stripe_shared::DisputeEnhancedEligibility,
    /// Whether evidence has been staged for this dispute.
    pub has_evidence: bool,
    /// Whether the last evidence submission was submitted past the due date.
    /// Defaults to `false` if no evidence submissions have occurred.
    /// If `true`, then delivery of the latest evidence is *not* guaranteed.
    pub past_due: bool,
    /// The number of times evidence has been submitted. Typically, you may only submit evidence once.
    pub submission_count: u64,
    /// Whether the dispute was submitted manually, with Smart Disputes, or not submitted.
    pub submission_method: DisputeEvidenceDetailsSubmissionMethod,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEvidenceDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputeEvidenceDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputeEvidenceDetailsBuilder {
    due_by: Option<Option<stripe_types::Timestamp>>,
    enhanced_eligibility: Option<stripe_shared::DisputeEnhancedEligibility>,
    has_evidence: Option<bool>,
    past_due: Option<bool>,
    submission_count: Option<u64>,
    submission_method: Option<DisputeEvidenceDetailsSubmissionMethod>,
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

    impl Deserialize for DisputeEvidenceDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeEvidenceDetails>,
        builder: DisputeEvidenceDetailsBuilder,
    }

    impl Visitor for Place<DisputeEvidenceDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputeEvidenceDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeEvidenceDetailsBuilder {
        type Out = DisputeEvidenceDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "due_by" => Deserialize::begin(&mut self.due_by),
                "enhanced_eligibility" => Deserialize::begin(&mut self.enhanced_eligibility),
                "has_evidence" => Deserialize::begin(&mut self.has_evidence),
                "past_due" => Deserialize::begin(&mut self.past_due),
                "submission_count" => Deserialize::begin(&mut self.submission_count),
                "submission_method" => Deserialize::begin(&mut self.submission_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                due_by: Some(None),
                enhanced_eligibility: None,
                has_evidence: None,
                past_due: None,
                submission_count: None,
                submission_method: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(due_by),
                Some(enhanced_eligibility),
                Some(has_evidence),
                Some(past_due),
                Some(submission_count),
                Some(submission_method),
            ) = (
                self.due_by,
                self.enhanced_eligibility.take(),
                self.has_evidence,
                self.past_due,
                self.submission_count,
                self.submission_method.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                due_by,
                enhanced_eligibility,
                has_evidence,
                past_due,
                submission_count,
                submission_method,
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

    impl ObjectDeser for DisputeEvidenceDetails {
        type Builder = DisputeEvidenceDetailsBuilder;
    }

    impl FromValueOpt for DisputeEvidenceDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeEvidenceDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "due_by" => b.due_by = FromValueOpt::from_value(v),
                    "enhanced_eligibility" => b.enhanced_eligibility = FromValueOpt::from_value(v),
                    "has_evidence" => b.has_evidence = FromValueOpt::from_value(v),
                    "past_due" => b.past_due = FromValueOpt::from_value(v),
                    "submission_count" => b.submission_count = FromValueOpt::from_value(v),
                    "submission_method" => b.submission_method = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether the dispute was submitted manually, with Smart Disputes, or not submitted.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisputeEvidenceDetailsSubmissionMethod {
    Manual,
    NotSubmitted,
    SmartDisputes,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisputeEvidenceDetailsSubmissionMethod {
    pub fn as_str(&self) -> &str {
        use DisputeEvidenceDetailsSubmissionMethod::*;
        match self {
            Manual => "manual",
            NotSubmitted => "not_submitted",
            SmartDisputes => "smart_disputes",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisputeEvidenceDetailsSubmissionMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeEvidenceDetailsSubmissionMethod::*;
        match s {
            "manual" => Ok(Manual),
            "not_submitted" => Ok(NotSubmitted),
            "smart_disputes" => Ok(SmartDisputes),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DisputeEvidenceDetailsSubmissionMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisputeEvidenceDetailsSubmissionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DisputeEvidenceDetailsSubmissionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEvidenceDetailsSubmissionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(DisputeEvidenceDetailsSubmissionMethod)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputeEvidenceDetailsSubmissionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DisputeEvidenceDetailsSubmissionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DisputeEvidenceDetailsSubmissionMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputeEvidenceDetailsSubmissionMethod::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DisputeEvidenceDetailsSubmissionMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputeEvidenceDetailsSubmissionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
