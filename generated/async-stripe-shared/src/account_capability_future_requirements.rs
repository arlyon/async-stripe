#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountCapabilityFutureRequirements {
    /// Fields that are due and can be resolved by providing the corresponding alternative fields instead.
    /// Multiple alternatives can reference the same `original_fields_due`.
    /// When this happens, any of these alternatives can serve as a pathway for attempting to resolve the fields.
    /// Additionally, providing `original_fields_due` again also serves as a pathway for attempting to resolve the fields.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Date on which `future_requirements` becomes the main `requirements` hash and `future_requirements` becomes empty.
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on the capability's enablement state prior to transitioning.
    pub current_deadline: Option<stripe_types::Timestamp>,
    /// Fields that need to be resolved to keep the capability enabled.
    /// If not resolved by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Vec<String>,
    /// This is typed as an enum for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is null because fields in `future_requirements` will never disable the account.
    pub disabled_reason: Option<AccountCapabilityFutureRequirementsDisabledReason>,
    /// Details about validation and verification failures for `due` requirements that must be resolved.
    pub errors: Vec<stripe_shared::AccountRequirementsError>,
    /// Fields you must collect when all thresholds are reached.
    /// As they become required, they appear in `currently_due` as well.
    pub eventually_due: Vec<String>,
    /// Fields that haven't been resolved by `requirements.current_deadline`.
    /// These fields need to be resolved to enable the capability on the account.
    /// `future_requirements.past_due` is a subset of `requirements.past_due`.
    pub past_due: Vec<String>,
    /// Fields that are being reviewed, or might become required depending on the results of a review.
    /// If the review fails, these fields can move to `eventually_due`, `currently_due`, `past_due` or `alternatives`.
    /// Fields might appear in `eventually_due`, `currently_due`, `past_due` or `alternatives` and in `pending_verification` if one verification fails but another is still pending.
    pub pending_verification: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountCapabilityFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountCapabilityFutureRequirements").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountCapabilityFutureRequirementsBuilder {
    alternatives: Option<Option<Vec<stripe_shared::AccountRequirementsAlternative>>>,
    current_deadline: Option<Option<stripe_types::Timestamp>>,
    currently_due: Option<Vec<String>>,
    disabled_reason: Option<Option<AccountCapabilityFutureRequirementsDisabledReason>>,
    errors: Option<Vec<stripe_shared::AccountRequirementsError>>,
    eventually_due: Option<Vec<String>>,
    past_due: Option<Vec<String>>,
    pending_verification: Option<Vec<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for AccountCapabilityFutureRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCapabilityFutureRequirements>,
        builder: AccountCapabilityFutureRequirementsBuilder,
    }

    impl Visitor for Place<AccountCapabilityFutureRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountCapabilityFutureRequirementsBuilder {
                    alternatives: Deserialize::default(),
                    current_deadline: Deserialize::default(),
                    currently_due: Deserialize::default(),
                    disabled_reason: Deserialize::default(),
                    errors: Deserialize::default(),
                    eventually_due: Deserialize::default(),
                    past_due: Deserialize::default(),
                    pending_verification: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alternatives" => Deserialize::begin(&mut self.builder.alternatives),
                "current_deadline" => Deserialize::begin(&mut self.builder.current_deadline),
                "currently_due" => Deserialize::begin(&mut self.builder.currently_due),
                "disabled_reason" => Deserialize::begin(&mut self.builder.disabled_reason),
                "errors" => Deserialize::begin(&mut self.builder.errors),
                "eventually_due" => Deserialize::begin(&mut self.builder.eventually_due),
                "past_due" => Deserialize::begin(&mut self.builder.past_due),
                "pending_verification" => {
                    Deserialize::begin(&mut self.builder.pending_verification)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(alternatives),
                Some(current_deadline),
                Some(currently_due),
                Some(disabled_reason),
                Some(errors),
                Some(eventually_due),
                Some(past_due),
                Some(pending_verification),
            ) = (
                self.builder.alternatives.take(),
                self.builder.current_deadline,
                self.builder.currently_due.take(),
                self.builder.disabled_reason.take(),
                self.builder.errors.take(),
                self.builder.eventually_due.take(),
                self.builder.past_due.take(),
                self.builder.pending_verification.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(AccountCapabilityFutureRequirements {
                alternatives,
                current_deadline,
                currently_due,
                disabled_reason,
                errors,
                eventually_due,
                past_due,
                pending_verification,
            });
            Ok(())
        }
    }
};
/// This is typed as an enum for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is null because fields in `future_requirements` will never disable the account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountCapabilityFutureRequirementsDisabledReason {
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
impl AccountCapabilityFutureRequirementsDisabledReason {
    pub fn as_str(&self) -> &str {
        use AccountCapabilityFutureRequirementsDisabledReason::*;
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

impl std::str::FromStr for AccountCapabilityFutureRequirementsDisabledReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilityFutureRequirementsDisabledReason::*;
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
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountCapabilityFutureRequirementsDisabledReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountCapabilityFutureRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountCapabilityFutureRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountCapabilityFutureRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountCapabilityFutureRequirementsDisabledReason))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountCapabilityFutureRequirementsDisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for AccountCapabilityFutureRequirementsDisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<AccountCapabilityFutureRequirementsDisabledReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AccountCapabilityFutureRequirementsDisabledReason::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountCapabilityFutureRequirementsDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
