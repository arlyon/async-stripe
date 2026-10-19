#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationThreeDSecure {
    /// The outcome of the 3D Secure authentication request.
    pub result: stripe_shared::IssuingAuthorizationThreeDSecureResult,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationThreeDSecure").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationThreeDSecureBuilder {
    result: Option<stripe_shared::IssuingAuthorizationThreeDSecureResult>,
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
            Self { result: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(result),) = (self.result.take(),) else {
                return None;
            };
            Some(Self::Out { result })
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
pub type IssuingAuthorizationThreeDSecureResult =
    stripe_shared::IssuingAuthorizationThreeDSecureResult;
