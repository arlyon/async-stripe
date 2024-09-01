/// Represents a reader action to process a payment intent
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceProcessPaymentIntentAction {
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
    pub process_config: Option<stripe_terminal::TerminalReaderReaderResourceProcessConfig>,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceProcessPaymentIntentActionBuilder {
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    process_config: Option<Option<stripe_terminal::TerminalReaderReaderResourceProcessConfig>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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
                builder:
                    TerminalReaderReaderResourceProcessPaymentIntentActionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceProcessPaymentIntentActionBuilder {
        type Out = TerminalReaderReaderResourceProcessPaymentIntentAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "process_config" => Deserialize::begin(&mut self.process_config),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payment_intent: Deserialize::default(), process_config: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payment_intent), Some(process_config)) =
                (self.payment_intent.take(), self.process_config)
            else {
                return None;
            };
            Some(Self::Out { payment_intent, process_config })
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

    impl ObjectDeser for TerminalReaderReaderResourceProcessPaymentIntentAction {
        type Builder = TerminalReaderReaderResourceProcessPaymentIntentActionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceProcessPaymentIntentAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TerminalReaderReaderResourceProcessPaymentIntentActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "process_config" => b.process_config = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
