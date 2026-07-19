/// Represents a reader action to collect a payment method
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceCollectPaymentMethodAction {
    pub collect_config: Option<stripe_terminal::TerminalReaderReaderResourceCollectConfig>,
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
    pub payment_method: Option<stripe_shared::PaymentMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceCollectPaymentMethodAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceCollectPaymentMethodAction")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceCollectPaymentMethodActionBuilder {
    collect_config: Option<Option<stripe_terminal::TerminalReaderReaderResourceCollectConfig>>,
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    payment_method: Option<Option<stripe_shared::PaymentMethod>>,
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

    impl Deserialize for TerminalReaderReaderResourceCollectPaymentMethodAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceCollectPaymentMethodAction>,
        builder: TerminalReaderReaderResourceCollectPaymentMethodActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceCollectPaymentMethodAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceCollectPaymentMethodActionBuilder {
                    collect_config: Deserialize::default(),
                    payment_intent: Deserialize::default(),
                    payment_method: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "collect_config" => Deserialize::begin(&mut self.builder.collect_config),
                "payment_intent" => Deserialize::begin(&mut self.builder.payment_intent),
                "payment_method" => Deserialize::begin(&mut self.builder.payment_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(collect_config), Some(payment_intent), Some(payment_method)) = (
                self.builder.collect_config,
                self.builder.payment_intent.take(),
                self.builder.payment_method.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceCollectPaymentMethodAction {
                collect_config,
                payment_intent,
                payment_method,
            });
            Ok(())
        }
    }
};
