#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEvidenceVisaCompellingEvidence3 {
    /// Disputed transaction details for Visa Compelling Evidence 3.0 evidence submission.
    pub disputed_transaction:
        Option<stripe_shared::DisputeVisaCompellingEvidence3DisputedTransaction>,
    /// List of exactly two prior undisputed transaction objects for Visa Compelling Evidence 3.0 evidence submission.
    pub prior_undisputed_transactions:
        Vec<stripe_shared::DisputeVisaCompellingEvidence3PriorUndisputedTransaction>,
}
#[doc(hidden)]
pub struct DisputeEnhancedEvidenceVisaCompellingEvidence3Builder {
    disputed_transaction:
        Option<Option<stripe_shared::DisputeVisaCompellingEvidence3DisputedTransaction>>,
    prior_undisputed_transactions:
        Option<Vec<stripe_shared::DisputeVisaCompellingEvidence3PriorUndisputedTransaction>>,
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

    impl Deserialize for DisputeEnhancedEvidenceVisaCompellingEvidence3 {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeEnhancedEvidenceVisaCompellingEvidence3>,
        builder: DisputeEnhancedEvidenceVisaCompellingEvidence3Builder,
    }

    impl Visitor for Place<DisputeEnhancedEvidenceVisaCompellingEvidence3> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputeEnhancedEvidenceVisaCompellingEvidence3Builder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeEnhancedEvidenceVisaCompellingEvidence3Builder {
        type Out = DisputeEnhancedEvidenceVisaCompellingEvidence3;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disputed_transaction" => Deserialize::begin(&mut self.disputed_transaction),
                "prior_undisputed_transactions" => {
                    Deserialize::begin(&mut self.prior_undisputed_transactions)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                disputed_transaction: Deserialize::default(),
                prior_undisputed_transactions: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(disputed_transaction), Some(prior_undisputed_transactions)) =
                (self.disputed_transaction.take(), self.prior_undisputed_transactions.take())
            else {
                return None;
            };
            Some(Self::Out { disputed_transaction, prior_undisputed_transactions })
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

    impl ObjectDeser for DisputeEnhancedEvidenceVisaCompellingEvidence3 {
        type Builder = DisputeEnhancedEvidenceVisaCompellingEvidence3Builder;
    }

    impl FromValueOpt for DisputeEnhancedEvidenceVisaCompellingEvidence3 {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeEnhancedEvidenceVisaCompellingEvidence3Builder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "disputed_transaction" => b.disputed_transaction = FromValueOpt::from_value(v),
                    "prior_undisputed_transactions" => {
                        b.prior_undisputed_transactions = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
