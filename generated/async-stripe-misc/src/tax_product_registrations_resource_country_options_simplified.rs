#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsSimplified {
    /// Type of registration in `country`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsSimplifiedType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsSimplified {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductRegistrationsResourceCountryOptionsSimplified")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsSimplifiedBuilder {
    type_: Option<TaxProductRegistrationsResourceCountryOptionsSimplifiedType>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsSimplified {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
        builder: TaxProductRegistrationsResourceCountryOptionsSimplifiedBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsSimplified> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductRegistrationsResourceCountryOptionsSimplifiedBuilder {
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(type_),) = (self.builder.type_.take(),) else {
                return Ok(());
            };
            *self.out = Some(TaxProductRegistrationsResourceCountryOptionsSimplified { type_ });
            Ok(())
        }
    }
};
/// Type of registration in `country`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    Simplified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    pub fn as_str(&self) -> &str {
        use TaxProductRegistrationsResourceCountryOptionsSimplifiedType::*;
        match self {
            Simplified => "simplified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsSimplifiedType::*;
        match s {
            "simplified" => Ok(Simplified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductRegistrationsResourceCountryOptionsSimplifiedType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxProductRegistrationsResourceCountryOptionsSimplifiedType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TaxProductRegistrationsResourceCountryOptionsSimplifiedType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductRegistrationsResourceCountryOptionsSimplifiedType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
