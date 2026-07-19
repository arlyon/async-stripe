#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEvidenceVisaCompliance {
    /// A field acknowledging the fee incurred when countering a Visa compliance dispute.
    /// If this field is set to true, evidence can be submitted for the compliance dispute.
    /// Stripe collects a 500 USD (or local equivalent) amount to cover the network costs associated with resolving compliance disputes.
    /// Stripe refunds the 500 USD network fee if you win the dispute.
    pub fee_acknowledged: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEnhancedEvidenceVisaCompliance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputeEnhancedEvidenceVisaCompliance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputeEnhancedEvidenceVisaComplianceBuilder {
    fee_acknowledged: Option<bool>,
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

    impl Deserialize for DisputeEnhancedEvidenceVisaCompliance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeEnhancedEvidenceVisaCompliance>,
        builder: DisputeEnhancedEvidenceVisaComplianceBuilder,
    }

    impl Visitor for Place<DisputeEnhancedEvidenceVisaCompliance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputeEnhancedEvidenceVisaComplianceBuilder {
                    fee_acknowledged: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fee_acknowledged" => Deserialize::begin(&mut self.builder.fee_acknowledged),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(fee_acknowledged),) = (self.builder.fee_acknowledged,) else {
                return Ok(());
            };
            *self.out = Some(DisputeEnhancedEvidenceVisaCompliance { fee_acknowledged });
            Ok(())
        }
    }
};
