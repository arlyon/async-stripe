/// Collection of signals for this payment evaluation.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationSignals {
    pub fraudulent_payment: stripe_fraud::InsightsResourcesPaymentEvaluationSignalV2,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationSignals {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationSignals").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationSignalsBuilder {
    fraudulent_payment: Option<stripe_fraud::InsightsResourcesPaymentEvaluationSignalV2>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationSignals {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationSignals>,
        builder: InsightsResourcesPaymentEvaluationSignalsBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationSignals> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationSignalsBuilder {
                    fraudulent_payment: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fraudulent_payment" => Deserialize::begin(&mut self.builder.fraudulent_payment),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(fraudulent_payment),) = (self.builder.fraudulent_payment.take(),) else {
                return Ok(());
            };
            *self.out = Some(InsightsResourcesPaymentEvaluationSignals { fraudulent_payment });
            Ok(())
        }
    }
};
