#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountFutureRequirements {
    /// Fields that are due and can be resolved by providing the corresponding alternative fields instead.
    /// Many alternatives can list the same `original_fields_due`, and any of these alternatives can serve as a pathway for attempting to resolve the fields again.
    /// Re-providing `original_fields_due` also serves as a pathway for attempting to resolve the fields again.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Date on which `future_requirements` becomes the main `requirements` hash and `future_requirements` becomes empty.
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on its enablement state prior to transitioning.
    pub current_deadline: Option<stripe_types::Timestamp>,
    /// Fields that need to be resolved to keep the account enabled.
    /// If not resolved by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Option<Vec<String>>,
    /// This is typed as an enum for consistency with `requirements.disabled_reason`.
    pub disabled_reason: Option<AccountFutureRequirementsDisabledReason>,
    /// Details about validation and verification failures for `due` requirements that must be resolved.
    pub errors: Option<Vec<stripe_shared::AccountRequirementsError>>,
    /// Fields you must collect when all thresholds are reached.
    /// As they become required, they appear in `currently_due` as well.
    pub eventually_due: Option<Vec<String>>,
    /// Fields that haven't been resolved by `requirements.current_deadline`.
    /// These fields need to be resolved to enable the capability on the account.
    /// `future_requirements.past_due` is a subset of `requirements.past_due`.
    pub past_due: Option<Vec<String>>,
    /// Fields that are being reviewed, or might become required depending on the results of a review.
    /// If the review fails, these fields can move to `eventually_due`, `currently_due`, `past_due` or `alternatives`.
    /// Fields might appear in `eventually_due`, `currently_due`, `past_due` or `alternatives` and in `pending_verification` if one verification fails but another is still pending.
    pub pending_verification: Option<Vec<String>>,
}
#[doc(hidden)]
pub struct AccountFutureRequirementsBuilder {
    alternatives: Option<Option<Vec<stripe_shared::AccountRequirementsAlternative>>>,
    current_deadline: Option<Option<stripe_types::Timestamp>>,
    currently_due: Option<Option<Vec<String>>>,
    disabled_reason: Option<Option<AccountFutureRequirementsDisabledReason>>,
    errors: Option<Option<Vec<stripe_shared::AccountRequirementsError>>>,
    eventually_due: Option<Option<Vec<String>>>,
    past_due: Option<Option<Vec<String>>>,
    pending_verification: Option<Option<Vec<String>>>,
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

    impl Deserialize for AccountFutureRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountFutureRequirements>,
        builder: AccountFutureRequirementsBuilder,
    }

    impl Visitor for Place<AccountFutureRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountFutureRequirementsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountFutureRequirementsBuilder {
        type Out = AccountFutureRequirements;
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
                self.disabled_reason.take(),
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

    impl ObjectDeser for AccountFutureRequirements {
        type Builder = AccountFutureRequirementsBuilder;
    }

    impl FromValueOpt for AccountFutureRequirements {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountFutureRequirementsBuilder::deser_default();
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
/// This is typed as an enum for consistency with `requirements.disabled_reason`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountFutureRequirementsDisabledReason {
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
impl AccountFutureRequirementsDisabledReason {
    pub fn as_str(&self) -> &str {
        use AccountFutureRequirementsDisabledReason::*;
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

impl std::str::FromStr for AccountFutureRequirementsDisabledReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountFutureRequirementsDisabledReason::*;
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
                    "AccountFutureRequirementsDisabledReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountFutureRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountFutureRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountFutureRequirementsDisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountFutureRequirementsDisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountFutureRequirementsDisabledReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountFutureRequirementsDisabledReason::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountFutureRequirementsDisabledReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountFutureRequirementsDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
