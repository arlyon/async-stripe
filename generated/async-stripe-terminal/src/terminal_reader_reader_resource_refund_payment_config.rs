/// Represents a per-transaction override of a reader configuration
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceRefundPaymentConfig {
    /// Enable customer-initiated cancellation when refunding this payment.
    pub enable_customer_cancellation: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceRefundPaymentConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceRefundPaymentConfig").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceRefundPaymentConfigBuilder {
    enable_customer_cancellation: Option<Option<bool>>,
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
                builder: TerminalReaderReaderResourceRefundPaymentConfigBuilder {
                    enable_customer_cancellation: Deserialize::default(),
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
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enable_customer_cancellation),) =
                (self.builder.enable_customer_cancellation,)
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceRefundPaymentConfig {
                enable_customer_cancellation,
            });
            Ok(())
        }
    }
};
