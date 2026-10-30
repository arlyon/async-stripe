#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceReaderSecurityConfig {
    /// Passcode used to access a reader's admin menu.
    pub admin_menu_passcode: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceReaderSecurityConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfigurationConfigurationResourceReaderSecurityConfig")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceReaderSecurityConfigBuilder {
    admin_menu_passcode: Option<String>,
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

    impl Deserialize for TerminalConfigurationConfigurationResourceReaderSecurityConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceReaderSecurityConfig>,
        builder: TerminalConfigurationConfigurationResourceReaderSecurityConfigBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceReaderSecurityConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TerminalConfigurationConfigurationResourceReaderSecurityConfigBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceReaderSecurityConfigBuilder {
        type Out = TerminalConfigurationConfigurationResourceReaderSecurityConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "admin_menu_passcode" => Deserialize::begin(&mut self.admin_menu_passcode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { admin_menu_passcode: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(admin_menu_passcode),) = (self.admin_menu_passcode.take(),) else {
                return None;
            };
            Some(Self::Out { admin_menu_passcode })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceReaderSecurityConfig {
        type Builder = TerminalConfigurationConfigurationResourceReaderSecurityConfigBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceReaderSecurityConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalConfigurationConfigurationResourceReaderSecurityConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "admin_menu_passcode" => b.admin_menu_passcode = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
