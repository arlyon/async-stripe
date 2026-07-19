#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardFraudWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardFraudWarning").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardFraudWarningBuilder {
    started_at: Option<Option<stripe_types::Timestamp>>,
    type_: Option<Option<IssuingCardFraudWarningType>>,
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
                builder: IssuingCardFraudWarningBuilder {
                    started_at: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "started_at" => Deserialize::begin(&mut self.builder.started_at),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(started_at), Some(type_)) =
                (self.builder.started_at, self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingCardFraudWarning { started_at, type_ });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingCardFraudWarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardFraudWarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingCardFraudWarningType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingCardFraudWarningType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingCardFraudWarningType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardFraudWarningType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardFraudWarningType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
