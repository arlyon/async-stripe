/// Represents a reader action to process a payment intent
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceProcessPaymentIntentAction {
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
    pub process_config: Option<stripe_terminal::TerminalReaderReaderResourceProcessConfig>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceProcessPaymentIntentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceProcessPaymentIntentAction")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceProcessPaymentIntentActionBuilder {
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    process_config: Option<Option<stripe_terminal::TerminalReaderReaderResourceProcessConfig>>,
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

    impl Deserialize for TerminalReaderReaderResourceProcessPaymentIntentAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceProcessPaymentIntentAction>,
        builder: TerminalReaderReaderResourceProcessPaymentIntentActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceProcessPaymentIntentAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceProcessPaymentIntentActionBuilder {
                    payment_intent: Deserialize::default(),
                    process_config: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_intent" => Deserialize::begin(&mut self.builder.payment_intent),
                "process_config" => Deserialize::begin(&mut self.builder.process_config),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payment_intent), Some(process_config)) =
                (self.builder.payment_intent.take(), self.builder.process_config.take())
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceProcessPaymentIntentAction {
                payment_intent,
                process_config,
            });
            Ok(())
        }
    }
};
