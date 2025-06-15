#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountCardIssuingSettings {
    pub tos_acceptance: Option<stripe_shared::CardIssuingAccountTermsOfService>,
}
#[doc(hidden)]
pub struct AccountCardIssuingSettingsBuilder {
    tos_acceptance: Option<Option<stripe_shared::CardIssuingAccountTermsOfService>>,
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
                builder: AccountCardIssuingSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountCardIssuingSettingsBuilder {
        type Out = AccountCardIssuingSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tos_acceptance" => Deserialize::begin(&mut self.tos_acceptance),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { tos_acceptance: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(tos_acceptance),) = (self.tos_acceptance.take(),) else {
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
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for AccountCardIssuingSettings {
        type Builder = AccountCardIssuingSettingsBuilder;
    }

    impl FromValueOpt for AccountCardIssuingSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountCardIssuingSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "tos_acceptance" => b.tos_acceptance = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
