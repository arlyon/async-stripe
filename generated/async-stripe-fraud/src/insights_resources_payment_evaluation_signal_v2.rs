/// A payment evaluation signal with evaluated_at, risk_level, and score fields.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationSignalV2 {
    /// The time when this signal was evaluated.
    pub evaluated_at: stripe_types::Timestamp,
    /// Risk level of this signal, based on the score.
    pub risk_level: InsightsResourcesPaymentEvaluationSignalV2RiskLevel,
    /// Score for this insight.
    /// Possible values for evaluated payments are -1 and any value between 0 and 100.
    /// The value is returned with two decimal places.
    /// A score of -1 indicates a test integration and higher scores indicate a higher likelihood of the signal being true.
    pub score: f64,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationSignalV2Builder {
    evaluated_at: Option<stripe_types::Timestamp>,
    risk_level: Option<InsightsResourcesPaymentEvaluationSignalV2RiskLevel>,
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
            Self {
                evaluated_at: Deserialize::default(),
                risk_level: Deserialize::default(),
                score: Deserialize::default(),
            }
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
            *self.out = self.builder.take_out();
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    Elevated,
    Highest,
    Normal,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationSignalV2RiskLevel::*;
        match self {
            Elevated => "elevated",
            Highest => "highest",
            Normal => "normal",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationSignalV2RiskLevel::*;
        match s {
            "elevated" => Ok(Elevated),
            "highest" => Ok(Highest),
            "normal" => Ok(Normal),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationSignalV2RiskLevel"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InsightsResourcesPaymentEvaluationSignalV2RiskLevel> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationSignalV2RiskLevel::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InsightsResourcesPaymentEvaluationSignalV2RiskLevel);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
