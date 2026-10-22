#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureStandaloneResourceChallengeDetails {
    /// Type of challenge flow you requested for this 3DS Authentication.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ThreeDSecureStandaloneResourceChallengeDetailsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceChallengeDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureStandaloneResourceChallengeDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureStandaloneResourceChallengeDetailsBuilder {
    type_: Option<ThreeDSecureStandaloneResourceChallengeDetailsType>,
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

    impl Deserialize for ThreeDSecureStandaloneResourceChallengeDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureStandaloneResourceChallengeDetails>,
        builder: ThreeDSecureStandaloneResourceChallengeDetailsBuilder,
    }

    impl Visitor for Place<ThreeDSecureStandaloneResourceChallengeDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureStandaloneResourceChallengeDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureStandaloneResourceChallengeDetailsBuilder {
        type Out = ThreeDSecureStandaloneResourceChallengeDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(type_),) = (self.type_.take(),) else {
                return None;
            };
            Some(Self::Out { type_ })
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

    impl ObjectDeser for ThreeDSecureStandaloneResourceChallengeDetails {
        type Builder = ThreeDSecureStandaloneResourceChallengeDetailsBuilder;
    }

    impl FromValueOpt for ThreeDSecureStandaloneResourceChallengeDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureStandaloneResourceChallengeDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of challenge flow you requested for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureStandaloneResourceChallengeDetailsType {
    Mandated,
    Preferred,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureStandaloneResourceChallengeDetailsType {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureStandaloneResourceChallengeDetailsType::*;
        match self {
            Mandated => "mandated",
            Preferred => "preferred",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureStandaloneResourceChallengeDetailsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureStandaloneResourceChallengeDetailsType::*;
        match s {
            "mandated" => Ok(Mandated),
            "preferred" => Ok(Preferred),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureStandaloneResourceChallengeDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureStandaloneResourceChallengeDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceChallengeDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceChallengeDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureStandaloneResourceChallengeDetailsType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureStandaloneResourceChallengeDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureStandaloneResourceChallengeDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureStandaloneResourceChallengeDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThreeDSecureStandaloneResourceChallengeDetailsType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureStandaloneResourceChallengeDetailsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureStandaloneResourceChallengeDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
