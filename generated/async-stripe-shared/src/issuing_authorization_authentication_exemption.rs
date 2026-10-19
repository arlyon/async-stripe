#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationAuthenticationExemption {
    /// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
    pub claimed_by: stripe_shared::ClaimedBy,
    /// The specific exemption claimed for this authorization.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::IssuingAuthorizationAuthenticationExemptionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationAuthenticationExemption").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationAuthenticationExemptionBuilder {
    claimed_by: Option<stripe_shared::ClaimedBy>,
    type_: Option<stripe_shared::IssuingAuthorizationAuthenticationExemptionType>,
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

    impl Deserialize for IssuingAuthorizationAuthenticationExemption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationAuthenticationExemption>,
        builder: IssuingAuthorizationAuthenticationExemptionBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationAuthenticationExemption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationAuthenticationExemptionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationAuthenticationExemptionBuilder {
        type Out = IssuingAuthorizationAuthenticationExemption;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "claimed_by" => Deserialize::begin(&mut self.claimed_by),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { claimed_by: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(claimed_by), Some(type_)) = (self.claimed_by.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { claimed_by, type_ })
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

    impl ObjectDeser for IssuingAuthorizationAuthenticationExemption {
        type Builder = IssuingAuthorizationAuthenticationExemptionBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationAuthenticationExemption {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationAuthenticationExemptionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "claimed_by" => b.claimed_by = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
pub type IssuingAuthorizationAuthenticationExemptionClaimedBy = stripe_shared::ClaimedBy;
/// The specific exemption claimed for this authorization.
pub type IssuingAuthorizationAuthenticationExemptionType =
    stripe_shared::IssuingAuthorizationAuthenticationExemptionType;
