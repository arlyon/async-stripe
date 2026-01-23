/// User intervention raised event details attached to this payment evaluation
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationUserInterventionRaised {
    pub custom:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom>,
    /// Unique identifier for the user intervention event.
    pub key: String,
    /// Type of user intervention raised.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: InsightsResourcesPaymentEvaluationUserInterventionRaisedType,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationUserInterventionRaisedBuilder {
    custom: Option<
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom>,
    >,
    key: Option<String>,
    type_: Option<InsightsResourcesPaymentEvaluationUserInterventionRaisedType>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationUserInterventionRaised {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationUserInterventionRaised>,
        builder: InsightsResourcesPaymentEvaluationUserInterventionRaisedBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationUserInterventionRaised> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    InsightsResourcesPaymentEvaluationUserInterventionRaisedBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationUserInterventionRaisedBuilder {
        type Out = InsightsResourcesPaymentEvaluationUserInterventionRaised;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom" => Deserialize::begin(&mut self.custom),
                "key" => Deserialize::begin(&mut self.key),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                custom: Deserialize::default(),
                key: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(custom), Some(key), Some(type_)) =
                (self.custom.take(), self.key.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { custom, key, type_ })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationUserInterventionRaised {
        type Builder = InsightsResourcesPaymentEvaluationUserInterventionRaisedBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationUserInterventionRaised {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                InsightsResourcesPaymentEvaluationUserInterventionRaisedBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "custom" => b.custom = FromValueOpt::from_value(v),
                    "key" => b.key = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of user intervention raised.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    V3ds,
    Captcha,
    Custom,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationUserInterventionRaisedType::*;
        match self {
            V3ds => "3ds",
            Captcha => "captcha",
            Custom => "custom",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationUserInterventionRaisedType::*;
        match s {
            "3ds" => Ok(V3ds),
            "captcha" => Ok(Captcha),
            "custom" => Ok(Custom),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationUserInterventionRaisedType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationUserInterventionRaisedType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationUserInterventionRaisedType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InsightsResourcesPaymentEvaluationUserInterventionRaisedType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
