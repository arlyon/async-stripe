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
impl miniserde::Deserialize for GelatoSessionMatchingOptionsDob {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoSessionMatchingOptionsDob> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoSessionMatchingOptionsDob::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoSessionMatchingOptionsDob);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoSessionMatchingOptionsDob {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
