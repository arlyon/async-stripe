/// Represents a reader action to confirm a payment
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceConfirmPaymentIntentAction {
    pub confirm_config: Option<stripe_terminal::TerminalReaderReaderResourceConfirmConfig>,
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceConfirmPaymentIntentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceConfirmPaymentIntentAction")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceConfirmPaymentIntentActionBuilder {
    confirm_config: Option<Option<stripe_terminal::TerminalReaderReaderResourceConfirmConfig>>,
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
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

    impl Deserialize for TerminalReaderReaderResourceConfirmPaymentIntentAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceConfirmPaymentIntentAction>,
        builder: TerminalReaderReaderResourceConfirmPaymentIntentActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceConfirmPaymentIntentAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceConfirmPaymentIntentActionBuilder {
                    confirm_config: Deserialize::default(),
                    payment_intent: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "confirm_config" => Deserialize::begin(&mut self.builder.confirm_config),
                "payment_intent" => Deserialize::begin(&mut self.builder.payment_intent),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(confirm_config), Some(payment_intent)) =
                (self.builder.confirm_config.take(), self.builder.payment_intent.take())
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceConfirmPaymentIntentAction {
                confirm_config,
                payment_intent,
            });
            Ok(())
        }
    }
};
