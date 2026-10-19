/// How card details were read in this transaction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReadMethod {
    pub fn as_str(&self) -> &str {
        use ReadMethod::*;
        match self {
            ContactEmv => "contact_emv",
            ContactlessEmv => "contactless_emv",
            ContactlessMagstripeMode => "contactless_magstripe_mode",
            MagneticStripeFallback => "magnetic_stripe_fallback",
            MagneticStripeTrack2 => "magnetic_stripe_track2",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReadMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReadMethod::*;
        match s {
            "contact_emv" => Ok(ContactEmv),
            "contactless_emv" => Ok(ContactlessEmv),
            "contactless_magstripe_mode" => Ok(ContactlessMagstripeMode),
            "magnetic_stripe_fallback" => Ok(MagneticStripeFallback),
            "magnetic_stripe_track2" => Ok(MagneticStripeTrack2),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ReadMethod");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReadMethod)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReadMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ReadMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ReadMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReadMethod::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ReadMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReadMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
