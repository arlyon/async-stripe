#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationThreeDSecure {
    /// The outcome of the 3D Secure authentication request.
    pub result: IssuingAuthorizationThreeDSecureResult,
}
#[doc(hidden)]
pub struct IssuingAuthorizationThreeDSecureBuilder {
    result: Option<IssuingAuthorizationThreeDSecureResult>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingAuthorizationThreeDSecure {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationThreeDSecure>,
        builder: IssuingAuthorizationThreeDSecureBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationThreeDSecure> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationThreeDSecureBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationThreeDSecureBuilder {
        type Out = IssuingAuthorizationThreeDSecure;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "result" => Deserialize::begin(&mut self.result),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { result: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(result),) = (self.result,) else {
                return None;
            };
            Some(Self::Out { result })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingAuthorizationThreeDSecure {
        type Builder = IssuingAuthorizationThreeDSecureBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationThreeDSecure {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationThreeDSecureBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "result" => b.result = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The outcome of the 3D Secure authentication request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationThreeDSecureResult {
    AttemptAcknowledged,
    Authenticated,
    Failed,
    Required,
}
impl IssuingAuthorizationThreeDSecureResult {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationThreeDSecureResult::*;
        match self {
            AttemptAcknowledged => "attempt_acknowledged",
            Authenticated => "authenticated",
            Failed => "failed",
            Required => "required",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationThreeDSecureResult {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationThreeDSecureResult::*;
        match s {
            "attempt_acknowledged" => Ok(AttemptAcknowledged),
            "authenticated" => Ok(Authenticated),
            "failed" => Ok(Failed),
            "required" => Ok(Required),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationThreeDSecureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationThreeDSecureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationThreeDSecureResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationThreeDSecureResult {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationThreeDSecureResult> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingAuthorizationThreeDSecureResult::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationThreeDSecureResult);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationThreeDSecureResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingAuthorizationThreeDSecureResult")
        })
    }
}
