#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TrialUpdateBehavior {
    ContinueTrial,
    EndTrial,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TrialUpdateBehavior {
    pub fn as_str(&self) -> &str {
        use TrialUpdateBehavior::*;
        match self {
            ContinueTrial => "continue_trial",
            EndTrial => "end_trial",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TrialUpdateBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TrialUpdateBehavior::*;
        match s {
            "continue_trial" => Ok(ContinueTrial),
            "end_trial" => Ok(EndTrial),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TrialUpdateBehavior");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TrialUpdateBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TrialUpdateBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TrialUpdateBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TrialUpdateBehavior)).finish_non_exhaustive()
    }
}
impl serde::Serialize for TrialUpdateBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TrialUpdateBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TrialUpdateBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TrialUpdateBehavior::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TrialUpdateBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TrialUpdateBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
