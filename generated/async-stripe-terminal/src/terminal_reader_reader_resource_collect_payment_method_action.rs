/// Represents a reader action to collect a payment method
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceCollectPaymentMethodAction {
    pub collect_config: Option<stripe_terminal::TerminalReaderReaderResourceCollectConfig>,
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
    pub payment_method: Option<stripe_shared::PaymentMethod>,
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
                builder:
                    TerminalReaderReaderResourceCollectPaymentMethodActionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceCollectPaymentMethodActionBuilder {
        type Out = TerminalReaderReaderResourceCollectPaymentMethodAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "collect_config" => Deserialize::begin(&mut self.collect_config),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "payment_method" => Deserialize::begin(&mut self.payment_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                collect_config: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_method: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(collect_config), Some(payment_intent), Some(payment_method)) =
                (self.collect_config, self.payment_intent.take(), self.payment_method.take())
            else {
                return None;
            };
            Some(Self::Out { collect_config, payment_intent, payment_method })
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

    impl ObjectDeser for TerminalReaderReaderResourceCollectPaymentMethodAction {
        type Builder = TerminalReaderReaderResourceCollectPaymentMethodActionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceCollectPaymentMethodAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TerminalReaderReaderResourceCollectPaymentMethodActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "collect_config" => b.collect_config = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "payment_method" => b.payment_method = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
