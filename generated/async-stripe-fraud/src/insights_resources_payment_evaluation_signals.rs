/// Collection of signals for this payment evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationSignals {
    pub fraudulent_payment: stripe_fraud::InsightsResourcesPaymentEvaluationSignalV2,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationSignalsBuilder {
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
                "fraudulent_payment" => Deserialize::begin(&mut self.fraudulent_payment),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { fraudulent_payment: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(fraudulent_payment),) = (self.fraudulent_payment.take(),) else {
                return None;
            };
            Some(Self::Out { fraudulent_payment })
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
                    "fraudulent_payment" => b.fraudulent_payment = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
