#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Promotions {
    Auto,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl Promotions {
    pub fn as_str(&self) -> &str {
        use Promotions::*;
        match self {
            Auto => "auto",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Promotions {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Promotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Promotions");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Promotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Promotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Promotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Promotions)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Promotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Promotions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Promotions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(Promotions::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(Promotions);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Promotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
