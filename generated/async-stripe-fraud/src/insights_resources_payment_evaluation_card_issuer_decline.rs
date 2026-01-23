/// Provides Stripe Radar's evaluation of the likelihood that a payment will be declined by the card issuer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationCardIssuerDecline {
    /// Stripe Radar's evaluation of the likelihood that the payment will be declined by the card issuer.
    /// Scores range from 0 to 100, with higher values indicating a higher likelihood of decline.
    pub model_score: f64,
    /// Recommended action based on the model score. Possible values are `block` and `continue`.
    pub recommended_action: InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationCardIssuerDeclineBuilder {
    model_score: Option<f64>,
    recommended_action:
        Option<InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationCardIssuerDecline {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationCardIssuerDecline>,
        builder: InsightsResourcesPaymentEvaluationCardIssuerDeclineBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationCardIssuerDecline> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationCardIssuerDeclineBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationCardIssuerDeclineBuilder {
        type Out = InsightsResourcesPaymentEvaluationCardIssuerDecline;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "model_score" => Deserialize::begin(&mut self.model_score),
                "recommended_action" => Deserialize::begin(&mut self.recommended_action),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { model_score: Deserialize::default(), recommended_action: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(model_score), Some(recommended_action)) =
                (self.model_score, self.recommended_action.take())
            else {
                return None;
            };
            Some(Self::Out { model_score, recommended_action })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationCardIssuerDecline {
        type Builder = InsightsResourcesPaymentEvaluationCardIssuerDeclineBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationCardIssuerDecline {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationCardIssuerDeclineBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "model_score" => b.model_score = FromValueOpt::from_value(v),
                    "recommended_action" => b.recommended_action = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Recommended action based on the model score. Possible values are `block` and `continue`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction {
    Block,
    Continue,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction::*;
        match self {
            Block => "block",
            Continue => "continue",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction::*;
        match s {
            "block" => Ok(Block),
            "continue" => Ok(Continue),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InsightsResourcesPaymentEvaluationCardIssuerDeclineRecommendedAction
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
