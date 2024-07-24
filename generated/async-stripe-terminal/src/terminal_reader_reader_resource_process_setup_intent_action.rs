/// Represents a reader action to process a setup intent
#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct TerminalReaderReaderResourceProcessSetupIntentActionBuilder {
    generated_card: Option<Option<String>>,
    process_config: Option<Option<stripe_terminal::TerminalReaderReaderResourceProcessSetupConfig>>,
    setup_intent: Option<stripe_types::Expandable<stripe_shared::SetupIntent>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: TerminalReaderReaderResourceProcessSetupIntentActionBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceProcessSetupIntentActionBuilder {
        type Out = TerminalReaderReaderResourceProcessSetupIntentAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "generated_card" => Deserialize::begin(&mut self.generated_card),
                "process_config" => Deserialize::begin(&mut self.process_config),
                "setup_intent" => Deserialize::begin(&mut self.setup_intent),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                generated_card: Deserialize::default(),
                process_config: Deserialize::default(),
                setup_intent: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                generated_card: self.generated_card.take()?,
                process_config: self.process_config?,
                setup_intent: self.setup_intent.take()?,
            })
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

    impl ObjectDeser for TerminalReaderReaderResourceProcessSetupIntentAction {
        type Builder = TerminalReaderReaderResourceProcessSetupIntentActionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceProcessSetupIntentAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TerminalReaderReaderResourceProcessSetupIntentActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "generated_card" => b.generated_card = Some(FromValueOpt::from_value(v)?),
                    "process_config" => b.process_config = Some(FromValueOpt::from_value(v)?),
                    "setup_intent" => b.setup_intent = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
