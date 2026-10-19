/// The cardholder’s preferred locales (languages), ordered by preference.
/// Locales can be `de`, `en`, `es`, `fr`, or `it`.
/// This changes the language of the [3D Secure flow](https://docs.stripe.com/issuing/3d-secure) and one-time password messages sent to the cardholder.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PreferredLocales {
    De,
    En,
    Es,
    Fr,
    It,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PreferredLocales {
    pub fn as_str(&self) -> &str {
        use PreferredLocales::*;
        match self {
            De => "de",
            En => "en",
            Es => "es",
            Fr => "fr",
            It => "it",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PreferredLocales {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PreferredLocales::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "es" => Ok(Es),
            "fr" => Ok(Fr),
            "it" => Ok(It),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PreferredLocales");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PreferredLocales {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PreferredLocales {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PreferredLocales {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PreferredLocales)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PreferredLocales {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PreferredLocales {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PreferredLocales> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PreferredLocales::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PreferredLocales);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PreferredLocales {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
