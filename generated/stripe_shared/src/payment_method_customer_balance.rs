#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodCustomerBalance {}
#[doc(hidden)]
pub struct PaymentMethodCustomerBalanceBuilder {}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodCustomerBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCustomerBalance>,
        builder: PaymentMethodCustomerBalanceBuilder,
    }

    impl Visitor for Place<PaymentMethodCustomerBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodCustomerBalanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodCustomerBalanceBuilder {
        type Out = PaymentMethodCustomerBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {}
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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

    impl ObjectDeser for PaymentMethodCustomerBalance {
        type Builder = PaymentMethodCustomerBalanceBuilder;
    }

    impl FromValueOpt for PaymentMethodCustomerBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodCustomerBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};