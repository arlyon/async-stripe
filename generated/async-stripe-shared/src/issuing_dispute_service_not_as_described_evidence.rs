#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeServiceNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Date when order was canceled.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Reason for canceling the order.
    pub cancellation_reason: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Date when the product was received.
    pub received_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeServiceNotAsDescribedEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingDisputeServiceNotAsDescribedEvidence").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingDisputeServiceNotAsDescribedEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    cancellation_reason: Option<Option<String>>,
    explanation: Option<Option<String>>,
    received_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for IssuingDisputeServiceNotAsDescribedEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeServiceNotAsDescribedEvidence>,
        builder: IssuingDisputeServiceNotAsDescribedEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeServiceNotAsDescribedEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeServiceNotAsDescribedEvidenceBuilder {
                    additional_documentation: Deserialize::default(),
                    canceled_at: Deserialize::default(),
                    cancellation_reason: Deserialize::default(),
                    explanation: Deserialize::default(),
                    received_at: Deserialize::default(),
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
                "canceled_at" => Deserialize::begin(&mut self.builder.canceled_at),
                "cancellation_reason" => Deserialize::begin(&mut self.builder.cancellation_reason),
                "explanation" => Deserialize::begin(&mut self.builder.explanation),
                "received_at" => Deserialize::begin(&mut self.builder.received_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(additional_documentation),
                Some(canceled_at),
                Some(cancellation_reason),
                Some(explanation),
                Some(received_at),
            ) = (
                self.builder.additional_documentation.take(),
                self.builder.canceled_at,
                self.builder.cancellation_reason.take(),
                self.builder.explanation.take(),
                self.builder.received_at,
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingDisputeServiceNotAsDescribedEvidence {
                additional_documentation,
                canceled_at,
                cancellation_reason,
                explanation,
                received_at,
            });
            Ok(())
        }
    }
};
