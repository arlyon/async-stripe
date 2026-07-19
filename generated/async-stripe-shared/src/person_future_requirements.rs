#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonFutureRequirements {
    /// Fields that are due and can be resolved by providing the corresponding alternative fields instead.
    /// Many alternatives can list the same `original_fields_due`, and any of these alternatives can serve as a pathway for attempting to resolve the fields again.
    /// Re-providing `original_fields_due` also serves as a pathway for attempting to resolve the fields again.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Fields that need to be resolved to keep the person's account enabled.
    /// If not resolved by the account's `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash, and may immediately become `past_due`, but the account may also be given a grace period depending on the account's enablement state prior to transition.
    pub currently_due: Vec<String>,
    /// Details about validation and verification failures for `due` requirements that must be resolved.
    pub errors: Vec<stripe_shared::AccountRequirementsError>,
    /// Fields you must collect when all thresholds are reached.
    /// As they become required, they appear in `currently_due` as well, and the account's `future_requirements[current_deadline]` becomes set.
    pub eventually_due: Vec<String>,
    /// Fields that haven't been resolved by the account's `requirements.current_deadline`.
    /// These fields need to be resolved to enable the person's account.
    /// `future_requirements.past_due` is a subset of `requirements.past_due`.
    pub past_due: Vec<String>,
    /// Fields that are being reviewed, or might become required depending on the results of a review.
    /// If the review fails, these fields can move to `eventually_due`, `currently_due`, `past_due` or `alternatives`.
    /// Fields might appear in `eventually_due`, `currently_due`, `past_due` or `alternatives` and in `pending_verification` if one verification fails but another is still pending.
    pub pending_verification: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonFutureRequirements").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonFutureRequirementsBuilder {
    alternatives: Option<Option<Vec<stripe_shared::AccountRequirementsAlternative>>>,
    currently_due: Option<Vec<String>>,
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

    impl Deserialize for PersonFutureRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonFutureRequirements>,
        builder: PersonFutureRequirementsBuilder,
    }

    impl Visitor for Place<PersonFutureRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonFutureRequirementsBuilder {
                    alternatives: Deserialize::default(),
                    currently_due: Deserialize::default(),
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
                "currently_due" => Deserialize::begin(&mut self.builder.currently_due),
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
                Some(currently_due),
                Some(errors),
                Some(eventually_due),
                Some(past_due),
                Some(pending_verification),
            ) = (
                self.builder.alternatives.take(),
                self.builder.currently_due.take(),
                self.builder.errors.take(),
                self.builder.eventually_due.take(),
                self.builder.past_due.take(),
                self.builder.pending_verification.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PersonFutureRequirements {
                alternatives,
                currently_due,
                errors,
                eventually_due,
                past_due,
                pending_verification,
            });
            Ok(())
        }
    }
};
