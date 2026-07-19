/// Represents a reader action to process a setup intent
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceProcessSetupIntentAction {
    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    /// Only present if it was possible to generate a card PaymentMethod.
    pub generated_card: Option<String>,
    pub process_config: Option<stripe_terminal::TerminalReaderReaderResourceProcessSetupConfig>,
    /// Most recent SetupIntent processed by the reader.
    pub setup_intent: stripe_types::Expandable<stripe_shared::SetupIntent>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceProcessSetupIntentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceProcessSetupIntentAction")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceProcessSetupIntentActionBuilder {
    generated_card: Option<Option<String>>,
    process_config: Option<Option<stripe_terminal::TerminalReaderReaderResourceProcessSetupConfig>>,
    setup_intent: Option<stripe_types::Expandable<stripe_shared::SetupIntent>>,
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

    impl Deserialize for TerminalReaderReaderResourceProcessSetupIntentAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceProcessSetupIntentAction>,
        builder: TerminalReaderReaderResourceProcessSetupIntentActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceProcessSetupIntentAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceProcessSetupIntentActionBuilder {
                    generated_card: Deserialize::default(),
                    process_config: Deserialize::default(),
                    setup_intent: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "generated_card" => Deserialize::begin(&mut self.builder.generated_card),
                "process_config" => Deserialize::begin(&mut self.builder.process_config),
                "setup_intent" => Deserialize::begin(&mut self.builder.setup_intent),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(generated_card), Some(process_config), Some(setup_intent)) = (
                self.builder.generated_card.take(),
                self.builder.process_config,
                self.builder.setup_intent.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceProcessSetupIntentAction {
                generated_card,
                process_config,
                setup_intent,
            });
            Ok(())
        }
    }
};
