#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// Details about validation and verification failures for `due` requirements that must be resolved.
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
                alternatives: Deserialize::default(),
                currently_due: Deserialize::default(),
                errors: Deserialize::default(),
                eventually_due: Deserialize::default(),
                past_due: Deserialize::default(),
                pending_verification: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(alternatives),
                Some(currently_due),
                Some(errors),
                Some(eventually_due),
                Some(past_due),
                Some(pending_verification),
            ) = (
                self.alternatives.take(),
                self.currently_due.take(),
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
                currently_due,
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

    impl ObjectDeser for PersonRequirements {
        type Builder = PersonRequirementsBuilder;
    }

    impl FromValueOpt for PersonRequirements {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PersonRequirementsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "alternatives" => b.alternatives = FromValueOpt::from_value(v),
                    "currently_due" => b.currently_due = FromValueOpt::from_value(v),
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
