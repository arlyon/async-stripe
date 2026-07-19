#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSessionMatchingOptions {
    /// Strictness of the DOB matching policy to apply.
    pub dob: Option<GelatoSessionMatchingOptionsDob>,
    /// Strictness of the name matching policy to apply.
    pub name: Option<GelatoSessionMatchingOptionsName>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoSessionMatchingOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoSessionMatchingOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoSessionMatchingOptionsBuilder {
    dob: Option<Option<GelatoSessionMatchingOptionsDob>>,
    name: Option<Option<GelatoSessionMatchingOptionsName>>,
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

    impl Deserialize for GelatoSessionMatchingOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSessionMatchingOptions>,
        builder: GelatoSessionMatchingOptionsBuilder,
    }

    impl Visitor for Place<GelatoSessionMatchingOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoSessionMatchingOptionsBuilder {
                    dob: Deserialize::default(),
                    name: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dob" => Deserialize::begin(&mut self.builder.dob),
                "name" => Deserialize::begin(&mut self.builder.name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(dob), Some(name)) = (self.builder.dob.take(), self.builder.name.take())
            else {
                return Ok(());
            };
            *self.out = Some(GelatoSessionMatchingOptions { dob, name });
            Ok(())
        }
    }
};
/// Strictness of the DOB matching policy to apply.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoSessionMatchingOptionsDob {
    None,
    Similar,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoSessionMatchingOptionsDob {
    pub fn as_str(&self) -> &str {
        use GelatoSessionMatchingOptionsDob::*;
        match self {
            None => "none",
            Similar => "similar",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoSessionMatchingOptionsDob {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSessionMatchingOptionsDob::*;
        match s {
            "none" => Ok(None),
            "similar" => Ok(Similar),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "GelatoSessionMatchingOptionsDob"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoSessionMatchingOptionsDob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoSessionMatchingOptionsDob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoSessionMatchingOptionsDob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoSessionMatchingOptionsDob)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoSessionMatchingOptionsDob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for GelatoSessionMatchingOptionsDob {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoSessionMatchingOptionsDob> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoSessionMatchingOptionsDob::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoSessionMatchingOptionsDob {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Strictness of the name matching policy to apply.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoSessionMatchingOptionsName {
    None,
    Similar,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoSessionMatchingOptionsName {
    pub fn as_str(&self) -> &str {
        use GelatoSessionMatchingOptionsName::*;
        match self {
            None => "none",
            Similar => "similar",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoSessionMatchingOptionsName {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSessionMatchingOptionsName::*;
        match s {
            "none" => Ok(None),
            "similar" => Ok(Similar),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "GelatoSessionMatchingOptionsName"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoSessionMatchingOptionsName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoSessionMatchingOptionsName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoSessionMatchingOptionsName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoSessionMatchingOptionsName)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoSessionMatchingOptionsName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for GelatoSessionMatchingOptionsName {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoSessionMatchingOptionsName> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoSessionMatchingOptionsName::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoSessionMatchingOptionsName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
