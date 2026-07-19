#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ExternalAccountRequirements {
    /// Fields that need to be resolved to keep the external account enabled.
    /// If not resolved by `current_deadline`, these fields will appear in `past_due` as well, and the account is disabled.
    pub currently_due: Option<Vec<String>>,
    /// Details about validation and verification failures for `due` requirements that must be resolved.
    pub errors: Option<Vec<stripe_shared::AccountRequirementsError>>,
    /// Fields that haven't been resolved by `current_deadline`.
    /// These fields need to be resolved to enable the external account.
    pub past_due: Option<Vec<String>>,
    /// Fields that are being reviewed, or might become required depending on the results of a review.
    /// If the review fails, these fields can move to `eventually_due`, `currently_due`, `past_due` or `alternatives`.
    /// Fields might appear in `eventually_due`, `currently_due`, `past_due` or `alternatives` and in `pending_verification` if one verification fails but another is still pending.
    pub pending_verification: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ExternalAccountRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalAccountRequirements").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ExternalAccountRequirementsBuilder {
    currently_due: Option<Option<Vec<String>>>,
    errors: Option<Option<Vec<stripe_shared::AccountRequirementsError>>>,
    past_due: Option<Option<Vec<String>>>,
    pending_verification: Option<Option<Vec<String>>>,
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

    impl Deserialize for ExternalAccountRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ExternalAccountRequirements>,
        builder: ExternalAccountRequirementsBuilder,
    }

    impl Visitor for Place<ExternalAccountRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ExternalAccountRequirementsBuilder {
                    currently_due: Deserialize::default(),
                    errors: Deserialize::default(),
                    past_due: Deserialize::default(),
                    pending_verification: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currently_due" => Deserialize::begin(&mut self.builder.currently_due),
                "errors" => Deserialize::begin(&mut self.builder.errors),
                "past_due" => Deserialize::begin(&mut self.builder.past_due),
                "pending_verification" => {
                    Deserialize::begin(&mut self.builder.pending_verification)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(currently_due), Some(errors), Some(past_due), Some(pending_verification)) = (
                self.builder.currently_due.take(),
                self.builder.errors.take(),
                self.builder.past_due.take(),
                self.builder.pending_verification.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(ExternalAccountRequirements {
                currently_due,
                errors,
                past_due,
                pending_verification,
            });
            Ok(())
        }
    }
};
