/// Represents a reader action to confirm a payment
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceConfirmPaymentIntentAction {
    pub confirm_config: Option<stripe_terminal::TerminalReaderReaderResourceConfirmConfig>,
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceConfirmPaymentIntentActionBuilder {
    confirm_config: Option<Option<stripe_terminal::TerminalReaderReaderResourceConfirmConfig>>,
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
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
                builder:
                    TerminalReaderReaderResourceConfirmPaymentIntentActionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceConfirmPaymentIntentActionBuilder {
        type Out = TerminalReaderReaderResourceConfirmPaymentIntentAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "confirm_config" => Deserialize::begin(&mut self.confirm_config),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { confirm_config: Deserialize::default(), payment_intent: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(confirm_config), Some(payment_intent)) =
                (self.confirm_config.take(), self.payment_intent.take())
            else {
                return None;
            };
            Some(Self::Out { confirm_config, payment_intent })
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

    impl ObjectDeser for TerminalReaderReaderResourceConfirmPaymentIntentAction {
        type Builder = TerminalReaderReaderResourceConfirmPaymentIntentActionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceConfirmPaymentIntentAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TerminalReaderReaderResourceConfirmPaymentIntentActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "confirm_config" => b.confirm_config = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
