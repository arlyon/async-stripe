#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountPaypaySiteRestrictedSettings {
    /// File explaining the payment flow for your business.
    pub payment_flow_file: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountPaypaySiteRestrictedSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountPaypaySiteRestrictedSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountPaypaySiteRestrictedSettingsBuilder {
    payment_flow_file: Option<Option<String>>,
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

    impl Deserialize for AccountPaypaySiteRestrictedSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountPaypaySiteRestrictedSettings>,
        builder: AccountPaypaySiteRestrictedSettingsBuilder,
    }

    impl Visitor for Place<AccountPaypaySiteRestrictedSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountPaypaySiteRestrictedSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountPaypaySiteRestrictedSettingsBuilder {
        type Out = AccountPaypaySiteRestrictedSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_flow_file" => Deserialize::begin(&mut self.payment_flow_file),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payment_flow_file: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payment_flow_file),) = (self.payment_flow_file.take(),) else {
                return None;
            };
            Some(Self::Out { payment_flow_file })
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

    impl ObjectDeser for AccountPaypaySiteRestrictedSettings {
        type Builder = AccountPaypaySiteRestrictedSettingsBuilder;
    }

    impl FromValueOpt for AccountPaypaySiteRestrictedSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountPaypaySiteRestrictedSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_flow_file" => b.payment_flow_file = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
