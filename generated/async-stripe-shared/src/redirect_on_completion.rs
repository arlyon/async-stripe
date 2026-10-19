/// This parameter applies to `ui_mode: embedded_page`.
/// Learn more about the [redirect behavior](https://docs.stripe.com/payments/checkout/custom-success-page?payment-ui=embedded-form) of embedded sessions.
/// Defaults to `always`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RedirectOnCompletion {
    Always,
    IfRequired,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RedirectOnCompletion {
    pub fn as_str(&self) -> &str {
        use RedirectOnCompletion::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RedirectOnCompletion {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RedirectOnCompletion::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            "never" => Ok(Never),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "RedirectOnCompletion");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RedirectOnCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for RedirectOnCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RedirectOnCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(RedirectOnCompletion)).finish_non_exhaustive()
    }
}
impl serde::Serialize for RedirectOnCompletion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for RedirectOnCompletion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RedirectOnCompletion> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RedirectOnCompletion::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(RedirectOnCompletion);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RedirectOnCompletion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
