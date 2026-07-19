/// A payment evaluation signal with evaluated_at, risk_level, and score fields.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationSignalV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationSignalV2").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: InsightsResourcesPaymentEvaluationSignalV2Builder {
                    evaluated_at: Deserialize::default(),
                    risk_level: Deserialize::default(),
                    score: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "evaluated_at" => Deserialize::begin(&mut self.builder.evaluated_at),
                "risk_level" => Deserialize::begin(&mut self.builder.risk_level),
                "score" => Deserialize::begin(&mut self.builder.score),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(evaluated_at), Some(risk_level), Some(score)) =
                (self.builder.evaluated_at, self.builder.risk_level.take(), self.builder.score)
            else {
                return Ok(());
            };
            *self.out = Some(InsightsResourcesPaymentEvaluationSignalV2 {
                evaluated_at,
                risk_level,
                score,
            });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InsightsResourcesPaymentEvaluationSignalV2RiskLevel))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<InsightsResourcesPaymentEvaluationSignalV2RiskLevel> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationSignalV2RiskLevel::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
