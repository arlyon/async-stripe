#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountRequirementsDisabledReason {
    ActionRequiredRequestedCapabilities,
    Listed,
    Other,
    PlatformPaused,
    RejectedFraud,
    RejectedIncompleteVerification,
    RejectedListed,
    RejectedOther,
    RejectedPlatformFraud,
    RejectedPlatformOther,
    RejectedPlatformTermsOfService,
    RejectedTermsOfService,
    RequirementsPastDue,
    RequirementsPendingVerification,
    UnderReview,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountRequirementsDisabledReason {
    pub fn as_str(&self) -> &str {
        use AccountRequirementsDisabledReason::*;
        match self {
            ActionRequiredRequestedCapabilities => "action_required.requested_capabilities",
            Listed => "listed",
            Other => "other",
            PlatformPaused => "platform_paused",
            RejectedFraud => "rejected.fraud",
            RejectedIncompleteVerification => "rejected.incomplete_verification",
            RejectedListed => "rejected.listed",
            RejectedOther => "rejected.other",
            RejectedPlatformFraud => "rejected.platform_fraud",
            RejectedPlatformOther => "rejected.platform_other",
            RejectedPlatformTermsOfService => "rejected.platform_terms_of_service",
            RejectedTermsOfService => "rejected.terms_of_service",
            RequirementsPastDue => "requirements.past_due",
            RequirementsPendingVerification => "requirements.pending_verification",
            UnderReview => "under_review",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountRequirementsDisabledReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountRequirementsDisabledReason::*;
        match s {
            "action_required.requested_capabilities" => Ok(ActionRequiredRequestedCapabilities),
            "listed" => Ok(Listed),
            "other" => Ok(Other),
            "platform_paused" => Ok(PlatformPaused),
            "rejected.fraud" => Ok(RejectedFraud),
            "rejected.incomplete_verification" => Ok(RejectedIncompleteVerification),
            "rejected.listed" => Ok(RejectedListed),
            "rejected.other" => Ok(RejectedOther),
            "rejected.platform_fraud" => Ok(RejectedPlatformFraud),
            "rejected.platform_other" => Ok(RejectedPlatformOther),
            "rejected.platform_terms_of_service" => Ok(RejectedPlatformTermsOfService),
            "rejected.terms_of_service" => Ok(RejectedTermsOfService),
            "requirements.past_due" => Ok(RequirementsPastDue),
            "requirements.pending_verification" => Ok(RequirementsPendingVerification),
            "under_review" => Ok(UnderReview),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountRequirementsDisabledReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountRequirementsDisabledReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountRequirementsDisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountRequirementsDisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountRequirementsDisabledReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountRequirementsDisabledReason::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountRequirementsDisabledReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountRequirementsDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
