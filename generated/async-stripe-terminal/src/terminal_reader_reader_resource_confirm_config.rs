/// Represents a per-transaction override of a reader configuration
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceConfirmConfig {
    /// If the customer doesn't abandon authenticating the payment, they're redirected to this URL after completion.
    pub return_url: Option<String>,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceConfirmConfigBuilder {
    return_url: Option<Option<String>>,
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

    impl Deserialize for TerminalReaderReaderResourceConfirmConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceConfirmConfig>,
        builder: TerminalReaderReaderResourceConfirmConfigBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceConfirmConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceConfirmConfigBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceConfirmConfigBuilder {
        type Out = TerminalReaderReaderResourceConfirmConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "return_url" => Deserialize::begin(&mut self.return_url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { return_url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(return_url),) = (self.return_url.take(),) else {
                return None;
            };
            Some(Self::Out { return_url })
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

    impl ObjectDeser for TerminalReaderReaderResourceConfirmConfig {
        type Builder = TerminalReaderReaderResourceConfirmConfigBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceConfirmConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceConfirmConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "return_url" => b.return_url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
