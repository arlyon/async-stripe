#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsSepaDebit {
    pub mandate_options:
        Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsSepaDebitBuilder {
    mandate_options:
        Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentPaymentMethodOptionsSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsSepaDebit>,
        builder: SetupIntentPaymentMethodOptionsSepaDebitBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsSepaDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsSepaDebitBuilder {
        type Out = SetupIntentPaymentMethodOptionsSepaDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mandate_options" => Deserialize::begin(&mut self.mandate_options),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { mandate_options: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { mandate_options: self.mandate_options? })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsSepaDebit {
        type Builder = SetupIntentPaymentMethodOptionsSepaDebitBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptionsSepaDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentPaymentMethodOptionsSepaDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "mandate_options" => b.mandate_options = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
