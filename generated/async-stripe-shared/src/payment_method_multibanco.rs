#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodMultibanco {}
#[doc(hidden)]
pub struct PaymentMethodMultibancoBuilder {}

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

    impl Deserialize for PaymentMethodMultibanco {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodMultibanco>,
        builder: PaymentMethodMultibancoBuilder,
    }

    impl Visitor for Place<PaymentMethodMultibanco> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodMultibancoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodMultibancoBuilder {
        type Out = PaymentMethodMultibanco;
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

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentMethodMultibanco {
        type Builder = PaymentMethodMultibancoBuilder;
    }

    impl FromValueOpt for PaymentMethodMultibanco {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodMultibancoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
