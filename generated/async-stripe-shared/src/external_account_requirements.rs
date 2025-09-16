#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ExternalAccountRequirements {
    /// Fields that need to be collected to keep the external account enabled.
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Option<Vec<String>>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Option<Vec<stripe_shared::AccountRequirementsError>>,
    /// Fields that weren't collected by `current_deadline`.
    /// These fields need to be collected to enable the external account.
    pub past_due: Option<Vec<String>>,
    /// Fields that might become required depending on the results of verification or review.
    /// It's an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    /// Fields might appear in `eventually_due`, `currently_due`, or `past_due` and in `pending_verification` if verification fails but another verification is still pending.
    pub pending_verification: Option<Vec<String>>,
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: ExternalAccountRequirementsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ExternalAccountRequirementsBuilder {
        type Out = ExternalAccountRequirements;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currently_due" => Deserialize::begin(&mut self.currently_due),
                "errors" => Deserialize::begin(&mut self.errors),
                "past_due" => Deserialize::begin(&mut self.past_due),
                "pending_verification" => Deserialize::begin(&mut self.pending_verification),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                currently_due: Deserialize::default(),
                errors: Deserialize::default(),
                past_due: Deserialize::default(),
                pending_verification: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(currently_due), Some(errors), Some(past_due), Some(pending_verification)) = (
                self.currently_due.take(),
                self.errors.take(),
                self.past_due.take(),
                self.pending_verification.take(),
            ) else {
                return None;
            };
            Some(Self::Out { currently_due, errors, past_due, pending_verification })
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

    impl ObjectDeser for ExternalAccountRequirements {
        type Builder = ExternalAccountRequirementsBuilder;
    }

    impl FromValueOpt for ExternalAccountRequirements {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ExternalAccountRequirementsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "currently_due" => b.currently_due = FromValueOpt::from_value(v),
                    "errors" => b.errors = FromValueOpt::from_value(v),
                    "past_due" => b.past_due = FromValueOpt::from_value(v),
                    "pending_verification" => b.pending_verification = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
