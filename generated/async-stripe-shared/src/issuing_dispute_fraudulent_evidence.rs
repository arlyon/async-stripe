#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeFraudulentEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
}
#[doc(hidden)]
pub struct IssuingDisputeFraudulentEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    explanation: Option<Option<String>>,
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

    impl Deserialize for IssuingDisputeFraudulentEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeFraudulentEvidence>,
        builder: IssuingDisputeFraudulentEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeFraudulentEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeFraudulentEvidenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingDisputeFraudulentEvidenceBuilder {
        type Out = IssuingDisputeFraudulentEvidence;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_documentation" => {
                    Deserialize::begin(&mut self.additional_documentation)
                }
                "explanation" => Deserialize::begin(&mut self.explanation),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                additional_documentation: Deserialize::default(),
                explanation: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(additional_documentation), Some(explanation)) =
                (self.additional_documentation.take(), self.explanation.take())
            else {
                return None;
            };
            Some(Self::Out { additional_documentation, explanation })
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

    impl ObjectDeser for IssuingDisputeFraudulentEvidence {
        type Builder = IssuingDisputeFraudulentEvidenceBuilder;
    }

    impl FromValueOpt for IssuingDisputeFraudulentEvidence {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingDisputeFraudulentEvidenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "additional_documentation" => {
                        b.additional_documentation = FromValueOpt::from_value(v)
                    }
                    "explanation" => b.explanation = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
