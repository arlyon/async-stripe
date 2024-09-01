/// Represents a per-transaction override of a reader configuration
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceRefundPaymentConfig {
    /// Enable customer initiated cancellation when refunding this payment.
    pub enable_customer_cancellation: Option<bool>,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceRefundPaymentConfigBuilder {
    enable_customer_cancellation: Option<Option<bool>>,
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

    impl Deserialize for TerminalReaderReaderResourceRefundPaymentConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceRefundPaymentConfig>,
        builder: TerminalReaderReaderResourceRefundPaymentConfigBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceRefundPaymentConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceRefundPaymentConfigBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceRefundPaymentConfigBuilder {
        type Out = TerminalReaderReaderResourceRefundPaymentConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enable_customer_cancellation" => {
                    Deserialize::begin(&mut self.enable_customer_cancellation)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enable_customer_cancellation: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(enable_customer_cancellation),) = (self.enable_customer_cancellation,) else {
                return None;
            };
            Some(Self::Out { enable_customer_cancellation })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TerminalReaderReaderResourceRefundPaymentConfig {
        type Builder = TerminalReaderReaderResourceRefundPaymentConfigBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceRefundPaymentConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceRefundPaymentConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enable_customer_cancellation" => {
                        b.enable_customer_cancellation = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
