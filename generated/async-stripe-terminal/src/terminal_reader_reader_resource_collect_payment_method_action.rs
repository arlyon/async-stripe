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
    collect_config: Option<stripe_terminal::TerminalReaderReaderResourceCollectConfig>,
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    payment_method: Option<stripe_shared::PaymentMethod>,
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
                "collect_config" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.collect_config)
                }
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "payment_method" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.payment_method)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { collect_config: None, payment_intent: None, payment_method: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { collect_config, payment_intent: Some(payment_intent), payment_method } => {
                    Some(Self::Out { collect_config, payment_intent, payment_method })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TerminalReaderReaderResourceCollectPaymentMethodAction {
        type Builder = TerminalReaderReaderResourceCollectPaymentMethodActionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceCollectPaymentMethodAction {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
