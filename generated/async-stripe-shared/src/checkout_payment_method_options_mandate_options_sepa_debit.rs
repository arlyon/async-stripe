#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutPaymentMethodOptionsMandateOptionsSepaDebit {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'STRIPE'.
    pub reference_prefix: Option<String>,
}
#[doc(hidden)]
pub struct CheckoutPaymentMethodOptionsMandateOptionsSepaDebitBuilder {
    reference_prefix: Option<Option<String>>,
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

    impl Deserialize for CheckoutPaymentMethodOptionsMandateOptionsSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutPaymentMethodOptionsMandateOptionsSepaDebit>,
        builder: CheckoutPaymentMethodOptionsMandateOptionsSepaDebitBuilder,
    }

    impl Visitor for Place<CheckoutPaymentMethodOptionsMandateOptionsSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutPaymentMethodOptionsMandateOptionsSepaDebitBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for CheckoutPaymentMethodOptionsMandateOptionsSepaDebitBuilder {
        type Out = CheckoutPaymentMethodOptionsMandateOptionsSepaDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference_prefix" => Deserialize::begin(&mut self.reference_prefix),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { reference_prefix: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(reference_prefix),) = (self.reference_prefix.take(),) else {
                return None;
            };
            Some(Self::Out { reference_prefix })
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

    impl ObjectDeser for CheckoutPaymentMethodOptionsMandateOptionsSepaDebit {
        type Builder = CheckoutPaymentMethodOptionsMandateOptionsSepaDebitBuilder;
    }

    impl FromValueOpt for CheckoutPaymentMethodOptionsMandateOptionsSepaDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutPaymentMethodOptionsMandateOptionsSepaDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "reference_prefix" => b.reference_prefix = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
