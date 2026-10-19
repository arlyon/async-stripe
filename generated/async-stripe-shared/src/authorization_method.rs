#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AuthorizationMethod {
    Chip,
    Contactless,
    KeyedIn,
    Online,
    Swipe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AuthorizationMethod {
    pub fn as_str(&self) -> &str {
        use AuthorizationMethod::*;
        match self {
            Chip => "chip",
            Contactless => "contactless",
            KeyedIn => "keyed_in",
            Online => "online",
            Swipe => "swipe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AuthorizationMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AuthorizationMethod::*;
        match s {
            "chip" => Ok(Chip),
            "contactless" => Ok(Contactless),
            "keyed_in" => Ok(KeyedIn),
            "online" => Ok(Online),
            "swipe" => Ok(Swipe),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AuthorizationMethod");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AuthorizationMethod)).finish_non_exhaustive()
    }
}
impl serde::Serialize for AuthorizationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AuthorizationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AuthorizationMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AuthorizationMethod::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AuthorizationMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AuthorizationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
