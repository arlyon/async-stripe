#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountCapabilityFutureRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Date on which `future_requirements` becomes the main `requirements` hash and `future_requirements` becomes empty.
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on the capability's enablement state prior to transitioning.
    pub current_deadline: Option<stripe_types::Timestamp>,
    /// Fields that need to be collected to keep the capability enabled.
    /// If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Vec<String>,
    /// This is typed as an enum for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is null because fields in `future_requirements` will never disable the account.
    pub disabled_reason: Option<AccountCapabilityFutureRequirementsDisabledReason>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<stripe_shared::AccountRequirementsError>,
    /// Fields you must collect when all thresholds are reached.
    /// As they become required, they appear in `currently_due` as well.
    pub eventually_due: Vec<String>,
    /// Fields that weren't collected by `requirements.current_deadline`.
    /// These fields need to be collected to enable the capability on the account.
    /// New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Vec<String>,
    /// Fields that might become required depending on the results of verification or review.
    /// It's an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due` or `currently_due`.
    /// Fields might appear in `eventually_due` or `currently_due` and in `pending_verification` if verification fails but another verification is still pending.
    pub pending_verification: Vec<String>,
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: AccountCapabilityFutureRequirementsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountCapabilityFutureRequirementsBuilder {
        type Out = AccountCapabilityFutureRequirements;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alternatives" => Deserialize::begin(&mut self.alternatives),
                "current_deadline" => Deserialize::begin(&mut self.current_deadline),
                "currently_due" => Deserialize::begin(&mut self.currently_due),
                "disabled_reason" => Deserialize::begin(&mut self.disabled_reason),
                "errors" => Deserialize::begin(&mut self.errors),
                "eventually_due" => Deserialize::begin(&mut self.eventually_due),
                "past_due" => Deserialize::begin(&mut self.past_due),
                "pending_verification" => Deserialize::begin(&mut self.pending_verification),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                alternatives: Deserialize::default(),
                current_deadline: Deserialize::default(),
                currently_due: Deserialize::default(),
                disabled_reason: Deserialize::default(),
                errors: Deserialize::default(),
                eventually_due: Deserialize::default(),
                past_due: Deserialize::default(),
                pending_verification: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.alternatives.take(),
                self.current_deadline,
                self.currently_due.take(),
                self.disabled_reason,
                self.errors.take(),
                self.eventually_due.take(),
                self.past_due.take(),
                self.pending_verification.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                alternatives,
                current_deadline,
                currently_due,
                disabled_reason,
                errors,
                eventually_due,
                past_due,
                pending_verification,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for AccountCapabilityFutureRequirements {
        type Builder = AccountCapabilityFutureRequirementsBuilder;
    }

    impl FromValueOpt for AccountCapabilityFutureRequirements {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountCapabilityFutureRequirementsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "alternatives" => b.alternatives = FromValueOpt::from_value(v),
                    "current_deadline" => b.current_deadline = FromValueOpt::from_value(v),
                    "currently_due" => b.currently_due = FromValueOpt::from_value(v),
                    "disabled_reason" => b.disabled_reason = FromValueOpt::from_value(v),
                    "errors" => b.errors = FromValueOpt::from_value(v),
                    "eventually_due" => b.eventually_due = FromValueOpt::from_value(v),
                    "past_due" => b.past_due = FromValueOpt::from_value(v),
                    "pending_verification" => b.pending_verification = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// This is typed as an enum for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is null because fields in `future_requirements` will never disable the account.
#[derive(Copy, Clone, Eq, PartialEq)]
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
}
impl AccountCapabilityFutureRequirementsDisabledReason {
    pub fn as_str(self) -> &'static str {
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
        }
    }
}

impl std::str::FromStr for AccountCapabilityFutureRequirementsDisabledReason {
    type Err = stripe_types::StripeParseError;
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
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for AccountCapabilityFutureRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilityFutureRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for AccountCapabilityFutureRequirementsDisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountCapabilityFutureRequirementsDisabledReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AccountCapabilityFutureRequirementsDisabledReason::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountCapabilityFutureRequirementsDisabledReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountCapabilityFutureRequirementsDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for AccountCapabilityFutureRequirementsDisabledReason",
            )
        })
    }
}
