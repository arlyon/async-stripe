/// Represents a per-transaction override of a reader configuration
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceProcessConfig {
    /// Enable customer-initiated cancellation when processing this payment.
    pub enable_customer_cancellation: Option<bool>,
    /// If the customer doesn't abandon authenticating the payment, they're redirected to this URL after completion.
    pub return_url: Option<String>,
    /// Override showing a tipping selection screen on this transaction.
    pub skip_tipping: Option<bool>,
    pub tipping: Option<stripe_terminal::TerminalReaderReaderResourceTippingConfig>,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceProcessConfigBuilder {
    enable_customer_cancellation: Option<Option<bool>>,
    return_url: Option<Option<String>>,
    skip_tipping: Option<Option<bool>>,
    tipping: Option<Option<stripe_terminal::TerminalReaderReaderResourceTippingConfig>>,
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

    impl Deserialize for TerminalReaderReaderResourceProcessConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceProcessConfig>,
        builder: TerminalReaderReaderResourceProcessConfigBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceProcessConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceProcessConfigBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceProcessConfigBuilder {
        type Out = TerminalReaderReaderResourceProcessConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enable_customer_cancellation" => {
                    Deserialize::begin(&mut self.enable_customer_cancellation)
                }
                "return_url" => Deserialize::begin(&mut self.return_url),
                "skip_tipping" => Deserialize::begin(&mut self.skip_tipping),
                "tipping" => Deserialize::begin(&mut self.tipping),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                enable_customer_cancellation: Deserialize::default(),
                return_url: Deserialize::default(),
                skip_tipping: Deserialize::default(),
                tipping: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(enable_customer_cancellation),
                Some(return_url),
                Some(skip_tipping),
                Some(tipping),
            ) = (
                self.enable_customer_cancellation,
                self.return_url.take(),
                self.skip_tipping,
                self.tipping,
            )
            else {
                return None;
            };
            Some(Self::Out { enable_customer_cancellation, return_url, skip_tipping, tipping })
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

    impl ObjectDeser for TerminalReaderReaderResourceProcessConfig {
        type Builder = TerminalReaderReaderResourceProcessConfigBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceProcessConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceProcessConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enable_customer_cancellation" => {
                        b.enable_customer_cancellation = FromValueOpt::from_value(v)
                    }
                    "return_url" => b.return_url = FromValueOpt::from_value(v),
                    "skip_tipping" => b.skip_tipping = FromValueOpt::from_value(v),
                    "tipping" => b.tipping = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
