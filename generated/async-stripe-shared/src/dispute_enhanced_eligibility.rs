#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEligibility {
    pub visa_compelling_evidence_3:
        Option<stripe_shared::DisputeEnhancedEligibilityVisaCompellingEvidence3>,
    pub visa_compliance: Option<stripe_shared::DisputeEnhancedEligibilityVisaCompliance>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEnhancedEligibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputeEnhancedEligibility").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputeEnhancedEligibilityBuilder {
    visa_compelling_evidence_3:
        Option<Option<stripe_shared::DisputeEnhancedEligibilityVisaCompellingEvidence3>>,
    visa_compliance: Option<Option<stripe_shared::DisputeEnhancedEligibilityVisaCompliance>>,
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

    impl Deserialize for DisputeEnhancedEligibility {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeEnhancedEligibility>,
        builder: DisputeEnhancedEligibilityBuilder,
    }

    impl Visitor for Place<DisputeEnhancedEligibility> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputeEnhancedEligibilityBuilder {
                    visa_compelling_evidence_3: Deserialize::default(),
                    visa_compliance: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "visa_compelling_evidence_3" => {
                    Deserialize::begin(&mut self.builder.visa_compelling_evidence_3)
                }
                "visa_compliance" => Deserialize::begin(&mut self.builder.visa_compliance),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(visa_compelling_evidence_3), Some(visa_compliance)) = (
                self.builder.visa_compelling_evidence_3.take(),
                self.builder.visa_compliance.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(DisputeEnhancedEligibility { visa_compelling_evidence_3, visa_compliance });
            Ok(())
        }
    }
};
