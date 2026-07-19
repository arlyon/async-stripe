#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEnhancedEvidenceVisaCompellingEvidence3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputeEnhancedEvidenceVisaCompellingEvidence3").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: DisputeEnhancedEvidenceVisaCompellingEvidence3Builder {
                    disputed_transaction: Deserialize::default(),
                    prior_undisputed_transactions: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disputed_transaction" => {
                    Deserialize::begin(&mut self.builder.disputed_transaction)
                }
                "prior_undisputed_transactions" => {
                    Deserialize::begin(&mut self.builder.prior_undisputed_transactions)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(disputed_transaction), Some(prior_undisputed_transactions)) = (
                self.builder.disputed_transaction.take(),
                self.builder.prior_undisputed_transactions.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(DisputeEnhancedEvidenceVisaCompellingEvidence3 {
                disputed_transaction,
                prior_undisputed_transactions,
            });
            Ok(())
        }
    }
};
