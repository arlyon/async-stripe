#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeNoValidAuthorizationEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeNoValidAuthorizationEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingDisputeNoValidAuthorizationEvidence").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingDisputeNoValidAuthorizationEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    explanation: Option<Option<String>>,
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

    impl Deserialize for IssuingDisputeNoValidAuthorizationEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeNoValidAuthorizationEvidence>,
        builder: IssuingDisputeNoValidAuthorizationEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeNoValidAuthorizationEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeNoValidAuthorizationEvidenceBuilder {
                    additional_documentation: Deserialize::default(),
                    explanation: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_documentation" => {
                    Deserialize::begin(&mut self.builder.additional_documentation)
                }
                "explanation" => Deserialize::begin(&mut self.builder.explanation),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(additional_documentation), Some(explanation)) =
                (self.builder.additional_documentation.take(), self.builder.explanation.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingDisputeNoValidAuthorizationEvidence {
                additional_documentation,
                explanation,
            });
            Ok(())
        }
    }
};
