/// Represents a per-transaction override of a reader configuration
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceCollectConfig {
    /// Enable customer-initiated cancellation when processing this payment.
    pub enable_customer_cancellation: Option<bool>,
    /// Override showing a tipping selection screen on this transaction.
    pub skip_tipping: Option<bool>,
    pub tipping: Option<stripe_terminal::TerminalReaderReaderResourceTippingConfig>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceCollectConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceCollectConfig").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceCollectConfigBuilder {
    enable_customer_cancellation: Option<Option<bool>>,
    skip_tipping: Option<Option<bool>>,
    tipping: Option<Option<stripe_terminal::TerminalReaderReaderResourceTippingConfig>>,
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

    impl Deserialize for TerminalReaderReaderResourceCollectConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceCollectConfig>,
        builder: TerminalReaderReaderResourceCollectConfigBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceCollectConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceCollectConfigBuilder {
                    enable_customer_cancellation: Deserialize::default(),
                    skip_tipping: Deserialize::default(),
                    tipping: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enable_customer_cancellation" => {
                    Deserialize::begin(&mut self.builder.enable_customer_cancellation)
                }
                "skip_tipping" => Deserialize::begin(&mut self.builder.skip_tipping),
                "tipping" => Deserialize::begin(&mut self.builder.tipping),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enable_customer_cancellation), Some(skip_tipping), Some(tipping)) = (
                self.builder.enable_customer_cancellation,
                self.builder.skip_tipping,
                self.builder.tipping,
            ) else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceCollectConfig {
                enable_customer_cancellation,
                skip_tipping,
                tipping,
            });
            Ok(())
        }
    }
};
