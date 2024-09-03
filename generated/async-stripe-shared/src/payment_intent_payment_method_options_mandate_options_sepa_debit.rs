#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit {}
#[doc(hidden)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder {}

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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
        builder: PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder,
    }

    impl Visitor for Place<PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder {
        type Out = PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {}
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let () = () else {
                return None;
            };
            Some(Self::Out {})
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

    impl ObjectDeser for PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit {
        type Builder = PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder;
    }

    impl FromValueOpt for PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
