#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisabledReason {
    Other,
    PausedInactivity,
    PendingOnboarding,
    PendingReview,
    PlatformDisabled,
    PlatformPaused,
    RejectedInactivity,
    RejectedOther,
    RejectedUnsupportedBusiness,
    RequirementsFieldsNeeded,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisabledReason {
    pub fn as_str(&self) -> &str {
        use DisabledReason::*;
        match self {
            Other => "other",
            PausedInactivity => "paused.inactivity",
            PendingOnboarding => "pending.onboarding",
            PendingReview => "pending.review",
            PlatformDisabled => "platform_disabled",
            PlatformPaused => "platform_paused",
            RejectedInactivity => "rejected.inactivity",
            RejectedOther => "rejected.other",
            RejectedUnsupportedBusiness => "rejected.unsupported_business",
            RequirementsFieldsNeeded => "requirements.fields_needed",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisabledReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisabledReason::*;
        match s {
            "other" => Ok(Other),
            "paused.inactivity" => Ok(PausedInactivity),
            "pending.onboarding" => Ok(PendingOnboarding),
            "pending.review" => Ok(PendingReview),
            "platform_disabled" => Ok(PlatformDisabled),
            "platform_paused" => Ok(PlatformPaused),
            "rejected.inactivity" => Ok(RejectedInactivity),
            "rejected.other" => Ok(RejectedOther),
            "rejected.unsupported_business" => Ok(RejectedUnsupportedBusiness),
            "requirements.fields_needed" => Ok(RequirementsFieldsNeeded),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "DisabledReason");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(DisabledReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DisabledReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisabledReason::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DisabledReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
