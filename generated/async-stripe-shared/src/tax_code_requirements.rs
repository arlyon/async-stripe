#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxCodeRequirements {
    /// Describes whether a performance location is required for a successful tax calculation with a tax code.
    pub performance_location: TaxCodeRequirementsPerformanceLocation,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxCodeRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxCodeRequirements").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxCodeRequirementsBuilder {
    performance_location: Option<TaxCodeRequirementsPerformanceLocation>,
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

    impl Deserialize for TaxCodeRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxCodeRequirements>,
        builder: TaxCodeRequirementsBuilder,
    }

    impl Visitor for Place<TaxCodeRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxCodeRequirementsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxCodeRequirementsBuilder {
        type Out = TaxCodeRequirements;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "performance_location" => Deserialize::begin(&mut self.performance_location),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { performance_location: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(performance_location),) = (self.performance_location.take(),) else {
                return None;
            };
            Some(Self::Out { performance_location })
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

    impl ObjectDeser for TaxCodeRequirements {
        type Builder = TaxCodeRequirementsBuilder;
    }

    impl FromValueOpt for TaxCodeRequirements {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxCodeRequirementsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "performance_location" => b.performance_location = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Describes whether a performance location is required for a successful tax calculation with a tax code.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxCodeRequirementsPerformanceLocation {
    Optional,
    Required,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxCodeRequirementsPerformanceLocation {
    pub fn as_str(&self) -> &str {
        use TaxCodeRequirementsPerformanceLocation::*;
        match self {
            Optional => "optional",
            Required => "required",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxCodeRequirementsPerformanceLocation {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxCodeRequirementsPerformanceLocation::*;
        match s {
            "optional" => Ok(Optional),
            "required" => Ok(Required),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxCodeRequirementsPerformanceLocation"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxCodeRequirementsPerformanceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxCodeRequirementsPerformanceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxCodeRequirementsPerformanceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxCodeRequirementsPerformanceLocation)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxCodeRequirementsPerformanceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxCodeRequirementsPerformanceLocation {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxCodeRequirementsPerformanceLocation> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxCodeRequirementsPerformanceLocation::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxCodeRequirementsPerformanceLocation);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxCodeRequirementsPerformanceLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
