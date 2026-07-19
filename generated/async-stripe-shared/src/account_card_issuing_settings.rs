#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountCardIssuingSettings {
    pub tos_acceptance: Option<stripe_shared::CardIssuingAccountTermsOfService>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountCardIssuingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountCardIssuingSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountCardIssuingSettingsBuilder {
    tos_acceptance: Option<Option<stripe_shared::CardIssuingAccountTermsOfService>>,
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

    impl Deserialize for AccountCardIssuingSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCardIssuingSettings>,
        builder: AccountCardIssuingSettingsBuilder,
    }

    impl Visitor for Place<AccountCardIssuingSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountCardIssuingSettingsBuilder {
                    tos_acceptance: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tos_acceptance" => Deserialize::begin(&mut self.builder.tos_acceptance),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(tos_acceptance),) = (self.builder.tos_acceptance.take(),) else {
                return Ok(());
            };
            *self.out = Some(AccountCardIssuingSettings { tos_acceptance });
            Ok(())
        }
    }
};
