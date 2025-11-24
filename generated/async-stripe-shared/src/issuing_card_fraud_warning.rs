#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardFraudWarning {
    /// Timestamp of the most recent fraud warning.
    pub started_at: Option<stripe_types::Timestamp>,
    /// The type of fraud warning that most recently took place on this card.
    /// This field updates with every new fraud warning, so the value changes over time.
    /// If populated, cancel and reissue the card.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<IssuingCardFraudWarningType>,
}
#[doc(hidden)]
pub struct IssuingCardFraudWarningBuilder {
    started_at: Option<Option<stripe_types::Timestamp>>,
    type_: Option<Option<IssuingCardFraudWarningType>>,
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

    impl Deserialize for IssuingCardFraudWarning {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardFraudWarning>,
        builder: IssuingCardFraudWarningBuilder,
    }

    impl Visitor for Place<IssuingCardFraudWarning> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardFraudWarningBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardFraudWarningBuilder {
        type Out = IssuingCardFraudWarning;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "started_at" => Deserialize::begin(&mut self.started_at),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { started_at: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(started_at), Some(type_)) = (self.started_at, self.type_.take()) else {
                return None;
            };
            Some(Self::Out { started_at, type_ })
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

    impl ObjectDeser for IssuingCardFraudWarning {
        type Builder = IssuingCardFraudWarningBuilder;
    }

    impl FromValueOpt for IssuingCardFraudWarning {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardFraudWarningBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "started_at" => b.started_at = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of fraud warning that most recently took place on this card.
/// This field updates with every new fraud warning, so the value changes over time.
/// If populated, cancel and reissue the card.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingCardFraudWarningType {
    CardTestingExposure,
    FraudDisputeFiled,
    ThirdPartyReported,
    UserIndicatedFraud,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingCardFraudWarningType {
    pub fn as_str(&self) -> &str {
        use IssuingCardFraudWarningType::*;
        match self {
            CardTestingExposure => "card_testing_exposure",
            FraudDisputeFiled => "fraud_dispute_filed",
            ThirdPartyReported => "third_party_reported",
            UserIndicatedFraud => "user_indicated_fraud",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingCardFraudWarningType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardFraudWarningType::*;
        match s {
            "card_testing_exposure" => Ok(CardTestingExposure),
            "fraud_dispute_filed" => Ok(FraudDisputeFiled),
            "third_party_reported" => Ok(ThirdPartyReported),
            "user_indicated_fraud" => Ok(UserIndicatedFraud),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingCardFraudWarningType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingCardFraudWarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardFraudWarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardFraudWarningType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardFraudWarningType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardFraudWarningType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardFraudWarningType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardFraudWarningType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardFraudWarningType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
