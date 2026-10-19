/// Specifies how events are aggregated.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Formula {
    Count,
    Last,
    Sum,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl Formula {
    pub fn as_str(&self) -> &str {
        use Formula::*;
        match self {
            Count => "count",
            Last => "last",
            Sum => "sum",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Formula {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Formula::*;
        match s {
            "count" => Ok(Count),
            "last" => Ok(Last),
            "sum" => Ok(Sum),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Formula");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Formula)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Formula {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Formula {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Formula> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(Formula::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(Formula);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Formula {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
