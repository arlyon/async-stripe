#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CancellationReason {
    pub fn as_str(&self) -> &str {
        use CancellationReason::*;
        match self {
            Abandoned => "abandoned",
            Duplicate => "duplicate",
            RequestedByCustomer => "requested_by_customer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CancellationReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CancellationReason::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "duplicate" => Ok(Duplicate),
            "requested_by_customer" => Ok(RequestedByCustomer),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CancellationReason");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CancellationReason)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CancellationReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CancellationReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CancellationReason::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CancellationReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
