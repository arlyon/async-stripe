#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PageSize {
    A4,
    Auto,
    Letter,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PageSize {
    pub fn as_str(&self) -> &str {
        use PageSize::*;
        match self {
            A4 => "a4",
            Auto => "auto",
            Letter => "letter",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PageSize {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PageSize::*;
        match s {
            "a4" => Ok(A4),
            "auto" => Ok(Auto),
            "letter" => Ok(Letter),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PageSize");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PageSize)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PageSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PageSize {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PageSize> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PageSize::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PageSize);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PageSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
