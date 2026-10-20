#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountTreasurySettings {
    pub tos_acceptance: Option<stripe_shared::AccountTermsOfService>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountTreasurySettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountTreasurySettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountTreasurySettingsBuilder {
    tos_acceptance: Option<stripe_shared::AccountTermsOfService>,
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

    impl Deserialize for AccountTreasurySettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountTreasurySettings>,
        builder: AccountTreasurySettingsBuilder,
    }

    impl Visitor for Place<AccountTreasurySettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountTreasurySettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountTreasurySettingsBuilder {
        type Out = AccountTreasurySettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tos_acceptance" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.tos_acceptance)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { tos_acceptance: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (tos_acceptance,) = (self.tos_acceptance.take(),) else {
                return None;
            };
            Some(Self::Out { tos_acceptance })
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

    impl ObjectDeser for AccountTreasurySettings {
        type Builder = AccountTreasurySettingsBuilder;
    }

    impl FromValueOpt for AccountTreasurySettings {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
