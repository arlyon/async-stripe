#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSessionMatchingOptions {
    /// Strictness of the DOB matching policy to apply.
    pub dob: Option<GelatoSessionMatchingOptionsDob>,
    /// Strictness of the name matching policy to apply.
    pub name: Option<GelatoSessionMatchingOptionsName>,
}
#[doc(hidden)]
pub struct GelatoSessionMatchingOptionsBuilder {
    dob: Option<Option<GelatoSessionMatchingOptionsDob>>,
    name: Option<Option<GelatoSessionMatchingOptionsName>>,
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
                builder: GelatoSessionMatchingOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoSessionMatchingOptionsBuilder {
        type Out = GelatoSessionMatchingOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dob" => Deserialize::begin(&mut self.dob),
                "name" => Deserialize::begin(&mut self.name),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { dob: Deserialize::default(), name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(dob), Some(name)) = (self.dob, self.name) else {
                return None;
            };
            Some(Self::Out { dob, name })
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

    impl ObjectDeser for GelatoSessionMatchingOptions {
        type Builder = GelatoSessionMatchingOptionsBuilder;
    }

    impl FromValueOpt for GelatoSessionMatchingOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoSessionMatchingOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dob" => b.dob = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Strictness of the DOB matching policy to apply.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoSessionMatchingOptionsDob {
    None,
    Similar,
}
impl GelatoSessionMatchingOptionsDob {
    pub fn as_str(self) -> &'static str {
        use GelatoSessionMatchingOptionsDob::*;
        match self {
            None => "none",
            Similar => "similar",
        }
    }
}

impl std::str::FromStr for GelatoSessionMatchingOptionsDob {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSessionMatchingOptionsDob::*;
        match s {
            "none" => Ok(None),
            "similar" => Ok(Similar),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoSessionMatchingOptionsDob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSessionMatchingOptionsDob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for GelatoSessionMatchingOptionsDob {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoSessionMatchingOptionsDob> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(GelatoSessionMatchingOptionsDob::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoSessionMatchingOptionsDob);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoSessionMatchingOptionsDob {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for GelatoSessionMatchingOptionsDob")
        })
    }
}
/// Strictness of the name matching policy to apply.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoSessionMatchingOptionsName {
    None,
    Similar,
}
impl GelatoSessionMatchingOptionsName {
    pub fn as_str(self) -> &'static str {
        use GelatoSessionMatchingOptionsName::*;
        match self {
            None => "none",
            Similar => "similar",
        }
    }
}

impl std::str::FromStr for GelatoSessionMatchingOptionsName {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSessionMatchingOptionsName::*;
        match s {
            "none" => Ok(None),
            "similar" => Ok(Similar),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoSessionMatchingOptionsName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSessionMatchingOptionsName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for GelatoSessionMatchingOptionsName {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoSessionMatchingOptionsName> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(GelatoSessionMatchingOptionsName::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoSessionMatchingOptionsName);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoSessionMatchingOptionsName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for GelatoSessionMatchingOptionsName")
        })
    }
}
