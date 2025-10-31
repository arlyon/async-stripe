#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEligibility {
    pub visa_compelling_evidence_3:
        Option<stripe_shared::DisputeEnhancedEligibilityVisaCompellingEvidence3>,
    pub visa_compliance: Option<stripe_shared::DisputeEnhancedEligibilityVisaCompliance>,
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
                "visa_compelling_evidence_3" => {
                    Deserialize::begin(&mut self.visa_compelling_evidence_3)
                }
                "visa_compliance" => Deserialize::begin(&mut self.visa_compliance),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                visa_compelling_evidence_3: Deserialize::default(),
                visa_compliance: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(visa_compelling_evidence_3), Some(visa_compliance)) =
                (self.visa_compelling_evidence_3.take(), self.visa_compliance)
            else {
                return None;
            };
            Some(Self::Out { visa_compelling_evidence_3, visa_compliance })
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

    impl ObjectDeser for DisputeEnhancedEligibility {
        type Builder = DisputeEnhancedEligibilityBuilder;
    }

    impl FromValueOpt for DisputeEnhancedEligibility {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeEnhancedEligibilityBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "visa_compelling_evidence_3" => {
                        b.visa_compelling_evidence_3 = FromValueOpt::from_value(v)
                    }
                    "visa_compliance" => b.visa_compliance = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
