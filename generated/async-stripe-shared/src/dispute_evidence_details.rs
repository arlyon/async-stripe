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
                builder: DisputeEvidenceDetailsBuilder {
                    due_by: Deserialize::default(),
                    enhanced_eligibility: Deserialize::default(),
                    has_evidence: Deserialize::default(),
                    past_due: Deserialize::default(),
                    submission_count: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "due_by" => Deserialize::begin(&mut self.builder.due_by),
                "enhanced_eligibility" => {
                    Deserialize::begin(&mut self.builder.enhanced_eligibility)
                }
                "has_evidence" => Deserialize::begin(&mut self.builder.has_evidence),
                "past_due" => Deserialize::begin(&mut self.builder.past_due),
                "submission_count" => Deserialize::begin(&mut self.builder.submission_count),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(due_by),
                Some(enhanced_eligibility),
                Some(has_evidence),
                Some(past_due),
                Some(submission_count),
            ) = (
                self.builder.due_by,
                self.builder.enhanced_eligibility.take(),
                self.builder.has_evidence,
                self.builder.past_due,
                self.builder.submission_count,
            )
            else {
                return Ok(());
            };
            *self.out = Some(DisputeEvidenceDetails {
                due_by,
                enhanced_eligibility,
                has_evidence,
                past_due,
                submission_count,
            });
            Ok(())
        }
    }
};
