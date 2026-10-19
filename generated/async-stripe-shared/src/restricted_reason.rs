#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RestrictedReason {
    pub fn as_str(&self) -> &str {
        use RestrictedReason::*;
        match self {
            AlreadyReversed => "already_reversed",
            DeadlinePassed => "deadline_passed",
            NetworkRestricted => "network_restricted",
            Other => "other",
            SourceFlowRestricted => "source_flow_restricted",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RestrictedReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RestrictedReason::*;
        match s {
            "already_reversed" => Ok(AlreadyReversed),
            "deadline_passed" => Ok(DeadlinePassed),
            "network_restricted" => Ok(NetworkRestricted),
            "other" => Ok(Other),
            "source_flow_restricted" => Ok(SourceFlowRestricted),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "RestrictedReason");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for RestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(RestrictedReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RestrictedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for RestrictedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RestrictedReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RestrictedReason::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(RestrictedReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RestrictedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
