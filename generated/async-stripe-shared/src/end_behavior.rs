/// Behavior of the subscription schedule and underlying subscription when it ends.
/// Possible values are `release` or `cancel` with the default being `release`.
/// `release` will end the subscription schedule and keep the underlying subscription running.
/// `cancel` will end the subscription schedule and cancel the underlying subscription.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum EndBehavior {
    Cancel,
    None,
    Release,
    Renew,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl EndBehavior {
    pub fn as_str(&self) -> &str {
        use EndBehavior::*;
        match self {
            Cancel => "cancel",
            None => "none",
            Release => "release",
            Renew => "renew",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for EndBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EndBehavior::*;
        match s {
            "cancel" => Ok(Cancel),
            "none" => Ok(None),
            "release" => Ok(Release),
            "renew" => Ok(Renew),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "EndBehavior");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for EndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for EndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(EndBehavior)).finish_non_exhaustive()
    }
}
impl serde::Serialize for EndBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for EndBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<EndBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(EndBehavior::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(EndBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for EndBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
