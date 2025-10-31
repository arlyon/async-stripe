#[derive(Clone, Debug)]
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
                builder: IssuingDisputeServiceNotAsDescribedEvidenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingDisputeServiceNotAsDescribedEvidenceBuilder {
        type Out = IssuingDisputeServiceNotAsDescribedEvidence;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_documentation" => {
                    Deserialize::begin(&mut self.additional_documentation)
                }
                "canceled_at" => Deserialize::begin(&mut self.canceled_at),
                "cancellation_reason" => Deserialize::begin(&mut self.cancellation_reason),
                "explanation" => Deserialize::begin(&mut self.explanation),
                "received_at" => Deserialize::begin(&mut self.received_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                additional_documentation: Deserialize::default(),
                canceled_at: Deserialize::default(),
                cancellation_reason: Deserialize::default(),
                explanation: Deserialize::default(),
                received_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(additional_documentation),
                Some(canceled_at),
                Some(cancellation_reason),
                Some(explanation),
                Some(received_at),
            ) = (
                self.additional_documentation.take(),
                self.canceled_at,
                self.cancellation_reason.take(),
                self.explanation.take(),
                self.received_at,
            )
            else {
                return None;
            };
            Some(Self::Out {
                additional_documentation,
                canceled_at,
                cancellation_reason,
                explanation,
                received_at,
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

    impl ObjectDeser for IssuingDisputeServiceNotAsDescribedEvidence {
        type Builder = IssuingDisputeServiceNotAsDescribedEvidenceBuilder;
    }

    impl FromValueOpt for IssuingDisputeServiceNotAsDescribedEvidence {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingDisputeServiceNotAsDescribedEvidenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "additional_documentation" => {
                        b.additional_documentation = FromValueOpt::from_value(v)
                    }
                    "canceled_at" => b.canceled_at = FromValueOpt::from_value(v),
                    "cancellation_reason" => b.cancellation_reason = FromValueOpt::from_value(v),
                    "explanation" => b.explanation = FromValueOpt::from_value(v),
                    "received_at" => b.received_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
