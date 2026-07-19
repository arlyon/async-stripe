#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountRequirementsAlternative {
    /// Fields that can be provided to resolve all fields in `original_fields_due`.
    pub alternative_fields_due: Vec<String>,
    /// Fields that are due and can be resolved by providing all fields in `alternative_fields_due`.
    pub original_fields_due: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountRequirementsAlternative {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountRequirementsAlternative").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountRequirementsAlternativeBuilder {
    alternative_fields_due: Option<Vec<String>>,
    original_fields_due: Option<Vec<String>>,
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

    impl Deserialize for AccountRequirementsAlternative {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountRequirementsAlternative>,
        builder: AccountRequirementsAlternativeBuilder,
    }

    impl Visitor for Place<AccountRequirementsAlternative> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountRequirementsAlternativeBuilder {
                    alternative_fields_due: Deserialize::default(),
                    original_fields_due: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alternative_fields_due" => {
                    Deserialize::begin(&mut self.builder.alternative_fields_due)
                }
                "original_fields_due" => Deserialize::begin(&mut self.builder.original_fields_due),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(alternative_fields_due), Some(original_fields_due)) = (
                self.builder.alternative_fields_due.take(),
                self.builder.original_fields_due.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(AccountRequirementsAlternative {
                alternative_fields_due,
                original_fields_due,
            });
            Ok(())
        }
    }
};
