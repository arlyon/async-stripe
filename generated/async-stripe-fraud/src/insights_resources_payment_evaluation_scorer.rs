/// Scores, insights and recommended action for one scorer for this PaymentEvaluation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationScorer {
    /// Recommended action based on the risk score. Possible values are `block` and `continue`.
    pub recommended_action: InsightsResourcesPaymentEvaluationScorerRecommendedAction,
    /// Stripe Radar’s evaluation of the risk level of the payment.
    /// Possible values for evaluated payments are between 0 and 100, with higher scores indicating higher risk.
    pub risk_score: i64,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationScorerBuilder {
    recommended_action: Option<InsightsResourcesPaymentEvaluationScorerRecommendedAction>,
    risk_score: Option<i64>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationScorer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationScorer>,
        builder: InsightsResourcesPaymentEvaluationScorerBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationScorer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationScorerBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationScorerBuilder {
        type Out = InsightsResourcesPaymentEvaluationScorer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "recommended_action" => Deserialize::begin(&mut self.recommended_action),
                "risk_score" => Deserialize::begin(&mut self.risk_score),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { recommended_action: Deserialize::default(), risk_score: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(recommended_action), Some(risk_score)) =
                (self.recommended_action.take(), self.risk_score)
            else {
                return None;
            };
            Some(Self::Out { recommended_action, risk_score })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationScorer {
        type Builder = InsightsResourcesPaymentEvaluationScorerBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationScorer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationScorerBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "recommended_action" => b.recommended_action = FromValueOpt::from_value(v),
                    "risk_score" => b.risk_score = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Recommended action based on the risk score. Possible values are `block` and `continue`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationScorerRecommendedAction {
    Block,
    Continue,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationScorerRecommendedAction {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationScorerRecommendedAction::*;
        match self {
            Block => "block",
            Continue => "continue",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationScorerRecommendedAction {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationScorerRecommendedAction::*;
        match s {
            "block" => Ok(Block),
            "continue" => Ok(Continue),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationScorerRecommendedAction"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationScorerRecommendedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationScorerRecommendedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationScorerRecommendedAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationScorerRecommendedAction {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationScorerRecommendedAction>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationScorerRecommendedAction::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InsightsResourcesPaymentEvaluationScorerRecommendedAction
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationScorerRecommendedAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
