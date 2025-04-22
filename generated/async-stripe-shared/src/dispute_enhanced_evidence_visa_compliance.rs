#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEvidenceVisaCompliance {
    /// A field acknowledging the fee incurred when countering a Visa compliance dispute.
    /// If this field is set to true, evidence can be submitted for the compliance dispute.
    /// Stripe collects a 500 USD (or local equivalent) amount to cover the network costs associated with resolving compliance disputes.
    /// Stripe refunds the 500 USD network fee if you win the dispute.
    pub fee_acknowledged: bool,
}
#[doc(hidden)]
pub struct DisputeEnhancedEvidenceVisaComplianceBuilder {
    fee_acknowledged: Option<bool>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: DisputeEnhancedEvidenceVisaComplianceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeEnhancedEvidenceVisaComplianceBuilder {
        type Out = DisputeEnhancedEvidenceVisaCompliance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fee_acknowledged" => Deserialize::begin(&mut self.fee_acknowledged),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { fee_acknowledged: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(fee_acknowledged),) = (self.fee_acknowledged,) else {
                return None;
            };
            Some(Self::Out { fee_acknowledged })
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

    impl ObjectDeser for DisputeEnhancedEvidenceVisaCompliance {
        type Builder = DisputeEnhancedEvidenceVisaComplianceBuilder;
    }

    impl FromValueOpt for DisputeEnhancedEvidenceVisaCompliance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeEnhancedEvidenceVisaComplianceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fee_acknowledged" => b.fee_acknowledged = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
