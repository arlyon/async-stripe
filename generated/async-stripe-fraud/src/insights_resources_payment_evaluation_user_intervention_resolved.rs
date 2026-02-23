/// User Intervention Resolved Event details attached to this payment evaluation
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationUserInterventionResolved {
    /// Unique ID of this intervention. Use this to provide the result.
    pub key: String,
    /// Result of the intervention if it has been completed.
    pub outcome: Option<InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome>,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationUserInterventionResolvedBuilder {
    key: Option<String>,
    outcome: Option<Option<InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome>>,
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
                builder:
                    InsightsResourcesPaymentEvaluationUserInterventionResolvedBuilder::deser_default(
                    ),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationUserInterventionResolvedBuilder {
        type Out = InsightsResourcesPaymentEvaluationUserInterventionResolved;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "key" => Deserialize::begin(&mut self.key),
                "outcome" => Deserialize::begin(&mut self.outcome),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { key: Deserialize::default(), outcome: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(key), Some(outcome)) = (self.key.take(), self.outcome.take()) else {
                return None;
            };
            Some(Self::Out { key, outcome })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationUserInterventionResolved {
        type Builder = InsightsResourcesPaymentEvaluationUserInterventionResolvedBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationUserInterventionResolved {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                InsightsResourcesPaymentEvaluationUserInterventionResolvedBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "key" => b.key = FromValueOpt::from_value(v),
                    "outcome" => b.outcome = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome
);
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
