#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountPaypaySiteInDevelopmentSettings {
    /// Field to indicate that the website password has been provided.
    pub password_provided: Option<bool>,
    /// The username needed to access your business's website.
    pub username: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountPaypaySiteInDevelopmentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountPaypaySiteInDevelopmentSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountPaypaySiteInDevelopmentSettingsBuilder {
    password_provided: Option<Option<bool>>,
    username: Option<Option<String>>,
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

    impl Deserialize for AccountPaypaySiteInDevelopmentSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountPaypaySiteInDevelopmentSettings>,
        builder: AccountPaypaySiteInDevelopmentSettingsBuilder,
    }

    impl Visitor for Place<AccountPaypaySiteInDevelopmentSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountPaypaySiteInDevelopmentSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountPaypaySiteInDevelopmentSettingsBuilder {
        type Out = AccountPaypaySiteInDevelopmentSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "password_provided" => Deserialize::begin(&mut self.password_provided),
                "username" => Deserialize::begin(&mut self.username),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { password_provided: Some(None), username: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(password_provided), Some(username)) =
                (self.password_provided, self.username.take())
            else {
                return None;
            };
            Some(Self::Out { password_provided, username })
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

    impl ObjectDeser for AccountPaypaySiteInDevelopmentSettings {
        type Builder = AccountPaypaySiteInDevelopmentSettingsBuilder;
    }

    impl FromValueOpt for AccountPaypaySiteInDevelopmentSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountPaypaySiteInDevelopmentSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "password_provided" => b.password_provided = FromValueOpt::from_value(v),
                    "username" => b.username = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
