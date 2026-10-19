/// A payment evaluation signal with evaluated_at, risk_level, and score fields.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationSignalV2 {
    /// The time when this signal was evaluated.
    pub evaluated_at: stripe_types::Timestamp,
    /// Risk level of this signal, based on the score.
    pub risk_level: stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel,
    /// Score for this signal.
    /// Possible values for evaluated payments are between 0 and 100.
    /// The value is returned with two decimal places and higher scores indicate a higher likelihood of the signal being true.
    /// A score of -1 is returned when a model evaluation was not performed, such as requests from incomplete integrations.
    pub score: f64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationSignalV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationSignalV2").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationSignalV2Builder {
    evaluated_at: Option<stripe_types::Timestamp>,
    risk_level: Option<stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel>,
    score: Option<f64>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationSignalV2 {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationSignalV2>,
        builder: InsightsResourcesPaymentEvaluationSignalV2Builder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationSignalV2> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationSignalV2Builder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationSignalV2Builder {
        type Out = InsightsResourcesPaymentEvaluationSignalV2;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "evaluated_at" => Deserialize::begin(&mut self.evaluated_at),
                "risk_level" => Deserialize::begin(&mut self.risk_level),
                "score" => Deserialize::begin(&mut self.score),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { evaluated_at: None, risk_level: None, score: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(evaluated_at), Some(risk_level), Some(score)) =
                (self.evaluated_at, self.risk_level.take(), self.score)
            else {
                return None;
            };
            Some(Self::Out { evaluated_at, risk_level, score })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationSignalV2 {
        type Builder = InsightsResourcesPaymentEvaluationSignalV2Builder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationSignalV2 {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationSignalV2Builder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "evaluated_at" => b.evaluated_at = FromValueOpt::from_value(v),
                    "risk_level" => b.risk_level = FromValueOpt::from_value(v),
                    "score" => b.score = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Risk level of this signal, based on the score.
pub type InsightsResourcesPaymentEvaluationSignalV2RiskLevel =
    stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel;
