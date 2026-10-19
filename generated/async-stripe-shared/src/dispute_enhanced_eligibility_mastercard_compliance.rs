#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEligibilityMastercardCompliance {
    /// Mastercard compliance eligibility status.
    pub status: stripe_shared::DisputeEnhancedEligibilityVisaComplianceStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEnhancedEligibilityMastercardCompliance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputeEnhancedEligibilityMastercardCompliance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputeEnhancedEligibilityMastercardComplianceBuilder {
    status: Option<stripe_shared::DisputeEnhancedEligibilityVisaComplianceStatus>,
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

    impl Deserialize for DisputeEnhancedEligibilityMastercardCompliance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeEnhancedEligibilityMastercardCompliance>,
        builder: DisputeEnhancedEligibilityMastercardComplianceBuilder,
    }

    impl Visitor for Place<DisputeEnhancedEligibilityMastercardCompliance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputeEnhancedEligibilityMastercardComplianceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeEnhancedEligibilityMastercardComplianceBuilder {
        type Out = DisputeEnhancedEligibilityMastercardCompliance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { status: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(status),) = (self.status.take(),) else {
                return None;
            };
            Some(Self::Out { status })
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

    impl ObjectDeser for DisputeEnhancedEligibilityMastercardCompliance {
        type Builder = DisputeEnhancedEligibilityMastercardComplianceBuilder;
    }

    impl FromValueOpt for DisputeEnhancedEligibilityMastercardCompliance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeEnhancedEligibilityMastercardComplianceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Mastercard compliance eligibility status.
pub type DisputeEnhancedEligibilityMastercardComplianceStatus =
    stripe_shared::DisputeEnhancedEligibilityVisaComplianceStatus;
