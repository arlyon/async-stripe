#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEligibility {
    pub mastercard_compliance:
        Option<stripe_shared::DisputeEnhancedEligibilityMastercardCompliance>,
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
    mastercard_compliance: Option<stripe_shared::DisputeEnhancedEligibilityMastercardCompliance>,
    visa_compelling_evidence_3:
        Option<stripe_shared::DisputeEnhancedEligibilityVisaCompellingEvidence3>,
    visa_compliance: Option<stripe_shared::DisputeEnhancedEligibilityVisaCompliance>,
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
                builder: DisputeEnhancedEligibilityBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeEnhancedEligibilityBuilder {
        type Out = DisputeEnhancedEligibility;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mastercard_compliance" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.mastercard_compliance)
                }
                "visa_compelling_evidence_3" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.visa_compelling_evidence_3,
                ),
                "visa_compliance" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.visa_compliance)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                mastercard_compliance: None,
                visa_compelling_evidence_3: None,
                visa_compliance: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { mastercard_compliance, visa_compelling_evidence_3, visa_compliance } =
                core::mem::replace(self, Self::deser_default());
            Some(Self::Out { mastercard_compliance, visa_compelling_evidence_3, visa_compliance })
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

    impl ObjectDeser for DisputeEnhancedEligibility {
        type Builder = DisputeEnhancedEligibilityBuilder;
    }

    impl FromValueOpt for DisputeEnhancedEligibility {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
