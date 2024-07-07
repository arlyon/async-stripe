#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct IssuingDisputeDuplicateEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    card_statement: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    cash_receipt: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    check_image: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    explanation: Option<Option<String>>,
    original_transaction: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                "additional_documentation" => {
                    Deserialize::begin(&mut self.additional_documentation)
                }
                "card_statement" => Deserialize::begin(&mut self.card_statement),
                "cash_receipt" => Deserialize::begin(&mut self.cash_receipt),
                "check_image" => Deserialize::begin(&mut self.check_image),
                "explanation" => Deserialize::begin(&mut self.explanation),
                "original_transaction" => Deserialize::begin(&mut self.original_transaction),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                additional_documentation: Deserialize::default(),
                card_statement: Deserialize::default(),
                cash_receipt: Deserialize::default(),
                check_image: Deserialize::default(),
                explanation: Deserialize::default(),
                original_transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                additional_documentation: self.additional_documentation.take()?,
                card_statement: self.card_statement.take()?,
                cash_receipt: self.cash_receipt.take()?,
                check_image: self.check_image.take()?,
                explanation: self.explanation.take()?,
                original_transaction: self.original_transaction.take()?,
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

    impl ObjectDeser for IssuingDisputeDuplicateEvidence {
        type Builder = IssuingDisputeDuplicateEvidenceBuilder;
    }

    impl FromValueOpt for IssuingDisputeDuplicateEvidence {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingDisputeDuplicateEvidenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "additional_documentation" => {
                        b.additional_documentation = Some(FromValueOpt::from_value(v)?)
                    }
                    "card_statement" => b.card_statement = Some(FromValueOpt::from_value(v)?),
                    "cash_receipt" => b.cash_receipt = Some(FromValueOpt::from_value(v)?),
                    "check_image" => b.check_image = Some(FromValueOpt::from_value(v)?),
                    "explanation" => b.explanation = Some(FromValueOpt::from_value(v)?),
                    "original_transaction" => {
                        b.original_transaction = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
