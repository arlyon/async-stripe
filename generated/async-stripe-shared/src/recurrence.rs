/// Defines how the alert will behave.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Recurrence {
    OneTime,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl Recurrence {
    pub fn as_str(&self) -> &str {
        use Recurrence::*;
        match self {
            OneTime => "one_time",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Recurrence {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Recurrence::*;
        match s {
            "one_time" => Ok(OneTime),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Recurrence");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Recurrence)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Recurrence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Recurrence {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Recurrence> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(Recurrence::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(Recurrence);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Recurrence {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
