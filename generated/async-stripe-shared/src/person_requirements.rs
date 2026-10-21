#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonRequirements {
    /// Fields that are due and can be resolved by providing the corresponding alternative fields instead.
    /// Many alternatives can list the same `original_fields_due`, and any of these alternatives can serve as a pathway for attempting to resolve the fields again.
    /// Re-providing `original_fields_due` also serves as a pathway for attempting to resolve the fields again.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Fields that need to be resolved to keep the person's account enabled.
    /// If not resolved by the account's `current_deadline`, these fields will appear in `past_due` as well, and the account is disabled.
    pub currently_due: Vec<String>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<stripe_shared::AccountRequirementsError>,
    /// Fields you must collect when all thresholds are reached.
    /// As they become required, they appear in `currently_due` as well, and the account's `current_deadline` becomes set.
    pub eventually_due: Vec<String>,
    /// Fields that haven't been resolved by `current_deadline`.
    /// These fields need to be resolved to enable the person's account.
    pub past_due: Vec<String>,
    /// Fields that are being reviewed, or might become required depending on the results of a review.
    /// If the review fails, these fields can move to `eventually_due`, `currently_due`, `past_due` or `alternatives`.
    /// Fields might appear in `eventually_due`, `currently_due`, `past_due` or `alternatives` and in `pending_verification` if one verification fails but another is still pending.
    pub pending_verification: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonRequirements").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonRequirementsBuilder {
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

    impl Deserialize for PersonRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonRequirements>,
        builder: PersonRequirementsBuilder,
    }

    impl Visitor for Place<PersonRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonRequirementsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PersonRequirementsBuilder {
        type Out = PersonRequirements;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alternatives" => Deserialize::begin(&mut self.alternatives),
                "currently_due" => Deserialize::begin(&mut self.currently_due),
                "errors" => Deserialize::begin(&mut self.errors),
                "eventually_due" => Deserialize::begin(&mut self.eventually_due),
                "past_due" => Deserialize::begin(&mut self.past_due),
                "pending_verification" => Deserialize::begin(&mut self.pending_verification),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                alternatives: Some(None),
                currently_due: None,
                errors: None,
                eventually_due: None,
                past_due: None,
                pending_verification: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    alternatives: Some(alternatives),
                    currently_due: Some(currently_due),
                    errors: Some(errors),
                    eventually_due: Some(eventually_due),
                    past_due: Some(past_due),
                    pending_verification: Some(pending_verification),
                } => Some(Self::Out {
                    alternatives,
                    currently_due,
                    errors,
                    eventually_due,
                    past_due,
                    pending_verification,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PersonRequirements {
        type Builder = PersonRequirementsBuilder;
    }

    impl FromValueOpt for PersonRequirements {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
