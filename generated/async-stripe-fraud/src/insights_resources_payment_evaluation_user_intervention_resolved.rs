/// User Intervention Resolved Event details attached to this payment evaluation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationUserInterventionResolved {
    /// Unique ID of this intervention. Use this to provide the result.
    pub key: String,
    /// Result of the intervention if it has been completed.
    pub outcome: Option<InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationUserInterventionResolved {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationUserInterventionResolved")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationUserInterventionResolvedBuilder {
    key: Option<String>,
    outcome: Option<Option<InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome>>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationUserInterventionResolved {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationUserInterventionResolved>,
        builder: InsightsResourcesPaymentEvaluationUserInterventionResolvedBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationUserInterventionResolved> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationUserInterventionResolvedBuilder {
                    key: Deserialize::default(),
                    outcome: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "key" => Deserialize::begin(&mut self.builder.key),
                "outcome" => Deserialize::begin(&mut self.builder.outcome),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(key), Some(outcome)) = (self.builder.key.take(), self.builder.outcome.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(InsightsResourcesPaymentEvaluationUserInterventionResolved { key, outcome });
            Ok(())
        }
    }
};
/// Result of the intervention if it has been completed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    Abandoned,
    Failed,
    Passed,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::*;
        match self {
            Abandoned => "abandoned",
            Failed => "failed",
            Passed => "passed",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "failed" => Ok(Failed),
            "passed" => Ok(Passed),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
