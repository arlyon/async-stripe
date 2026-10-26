/// Contains details of the flow preference used for a standalone 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureStandaloneResourceFlowPreference {
    pub challenge: Option<stripe_payment::ThreeDSecureStandaloneResourceChallengeDetails>,
    pub data_share: Option<stripe_payment::ThreeDSecureStandaloneResourceDataShareDetails>,
    pub frictionless: Option<stripe_payment::ThreeDSecureStandaloneResourceFrictionlessDetails>,
    /// Type of flow you requested for this 3DS Authentication.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ThreeDSecureStandaloneResourceFlowPreferenceType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceFlowPreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureStandaloneResourceFlowPreference").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureStandaloneResourceFlowPreferenceBuilder {
    challenge: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceChallengeDetails>>,
    data_share: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceDataShareDetails>>,
    frictionless: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceFrictionlessDetails>>,
    type_: Option<ThreeDSecureStandaloneResourceFlowPreferenceType>,
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

    impl Deserialize for ThreeDSecureStandaloneResourceFlowPreference {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureStandaloneResourceFlowPreference>,
        builder: ThreeDSecureStandaloneResourceFlowPreferenceBuilder,
    }

    impl Visitor for Place<ThreeDSecureStandaloneResourceFlowPreference> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureStandaloneResourceFlowPreferenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureStandaloneResourceFlowPreferenceBuilder {
        type Out = ThreeDSecureStandaloneResourceFlowPreference;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "challenge" => Deserialize::begin(&mut self.challenge),
                "data_share" => Deserialize::begin(&mut self.data_share),
                "frictionless" => Deserialize::begin(&mut self.frictionless),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                challenge: Some(None),
                data_share: Some(None),
                frictionless: Some(None),
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(challenge), Some(data_share), Some(frictionless), Some(type_)) = (
                self.challenge.take(),
                self.data_share.take(),
                self.frictionless.take(),
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { challenge, data_share, frictionless, type_ })
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

    impl ObjectDeser for ThreeDSecureStandaloneResourceFlowPreference {
        type Builder = ThreeDSecureStandaloneResourceFlowPreferenceBuilder;
    }

    impl FromValueOpt for ThreeDSecureStandaloneResourceFlowPreference {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureStandaloneResourceFlowPreferenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "challenge" => b.challenge = FromValueOpt::from_value(v),
                    "data_share" => b.data_share = FromValueOpt::from_value(v),
                    "frictionless" => b.frictionless = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of flow you requested for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureStandaloneResourceFlowPreferenceType {
    Challenge,
    DataShare,
    Frictionless,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureStandaloneResourceFlowPreferenceType {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureStandaloneResourceFlowPreferenceType::*;
        match self {
            Challenge => "challenge",
            DataShare => "data_share",
            Frictionless => "frictionless",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureStandaloneResourceFlowPreferenceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureStandaloneResourceFlowPreferenceType::*;
        match s {
            "challenge" => Ok(Challenge),
            "data_share" => Ok(DataShare),
            "frictionless" => Ok(Frictionless),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureStandaloneResourceFlowPreferenceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureStandaloneResourceFlowPreferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceFlowPreferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceFlowPreferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureStandaloneResourceFlowPreferenceType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureStandaloneResourceFlowPreferenceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureStandaloneResourceFlowPreferenceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureStandaloneResourceFlowPreferenceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThreeDSecureStandaloneResourceFlowPreferenceType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureStandaloneResourceFlowPreferenceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureStandaloneResourceFlowPreferenceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
