#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeDuplicateEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    pub card_statement: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    pub cash_receipt: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    pub check_image: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of.
    /// Of the two or more transactions that are copies of each other, this is original undisputed one.
    pub original_transaction: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeDuplicateEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingDisputeDuplicateEvidence").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingDisputeDuplicateEvidenceBuilder {
    additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    card_statement: Option<stripe_types::Expandable<stripe_shared::File>>,
    cash_receipt: Option<stripe_types::Expandable<stripe_shared::File>>,
    check_image: Option<stripe_types::Expandable<stripe_shared::File>>,
    explanation: Option<Option<String>>,
    original_transaction: Option<Option<String>>,
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

    impl Deserialize for IssuingDisputeDuplicateEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeDuplicateEvidence>,
        builder: IssuingDisputeDuplicateEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeDuplicateEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeDuplicateEvidenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingDisputeDuplicateEvidenceBuilder {
        type Out = IssuingDisputeDuplicateEvidence;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_documentation" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.additional_documentation,
                ),
                "card_statement" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_statement)
                }
                "cash_receipt" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.cash_receipt)
                }
                "check_image" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.check_image)
                }
                "explanation" => Deserialize::begin(&mut self.explanation),
                "original_transaction" => Deserialize::begin(&mut self.original_transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                additional_documentation: None,
                card_statement: None,
                cash_receipt: None,
                check_image: None,
                explanation: Some(None),
                original_transaction: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                additional_documentation,
                card_statement,
                cash_receipt,
                check_image,
                Some(explanation),
                Some(original_transaction),
            ) = (
                self.additional_documentation.take(),
                self.card_statement.take(),
                self.cash_receipt.take(),
                self.check_image.take(),
                self.explanation.take(),
                self.original_transaction.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                additional_documentation,
                card_statement,
                cash_receipt,
                check_image,
                explanation,
                original_transaction,
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

    impl ObjectDeser for IssuingDisputeDuplicateEvidence {
        type Builder = IssuingDisputeDuplicateEvidenceBuilder;
    }

    impl FromValueOpt for IssuingDisputeDuplicateEvidence {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
