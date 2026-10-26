/// Collection of signals for this payment evaluation.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationSignals {
    /// The likelihood that this `PaymentEvaluation` results in an early fraud warning.
    pub early_fraud_warning: Option<stripe_fraud::InsightsResourcesPaymentEvaluationSignalV2>,
    /// The likelihood that this `PaymentEvaluation` results in a dispute with reason code `fraudulent`.
    pub fraudulent_dispute: Option<stripe_fraud::InsightsResourcesPaymentEvaluationSignalV2>,
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
    early_fraud_warning: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationSignalV2>>,
    fraudulent_dispute: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationSignalV2>>,
    fraudulent_payment: Option<stripe_fraud::InsightsResourcesPaymentEvaluationSignalV2>,
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
                builder: InsightsResourcesPaymentEvaluationSignalsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationSignalsBuilder {
        type Out = InsightsResourcesPaymentEvaluationSignals;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "early_fraud_warning" => Deserialize::begin(&mut self.early_fraud_warning),
                "fraudulent_dispute" => Deserialize::begin(&mut self.fraudulent_dispute),
                "fraudulent_payment" => Deserialize::begin(&mut self.fraudulent_payment),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                early_fraud_warning: Some(None),
                fraudulent_dispute: Some(None),
                fraudulent_payment: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(early_fraud_warning), Some(fraudulent_dispute), Some(fraudulent_payment)) = (
                self.early_fraud_warning.take(),
                self.fraudulent_dispute.take(),
                self.fraudulent_payment.take(),
            ) else {
                return None;
            };
            Some(Self::Out { early_fraud_warning, fraudulent_dispute, fraudulent_payment })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationSignals {
        type Builder = InsightsResourcesPaymentEvaluationSignalsBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationSignals {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationSignalsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "early_fraud_warning" => b.early_fraud_warning = FromValueOpt::from_value(v),
                    "fraudulent_dispute" => b.fraudulent_dispute = FromValueOpt::from_value(v),
                    "fraudulent_payment" => b.fraudulent_payment = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
