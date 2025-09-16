#[derive(Clone, Debug)]
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

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                due_by: Deserialize::default(),
                enhanced_eligibility: Deserialize::default(),
                has_evidence: Deserialize::default(),
                past_due: Deserialize::default(),
                submission_count: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(due_by),
                Some(enhanced_eligibility),
                Some(has_evidence),
                Some(past_due),
                Some(submission_count),
            ) = (
                self.due_by,
                self.enhanced_eligibility.take(),
                self.has_evidence,
                self.past_due,
                self.submission_count,
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
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
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

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
