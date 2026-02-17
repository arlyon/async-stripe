#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKonbini {
    /// If the payment succeeded, this contains the details of the convenience store where the payment was completed.
    pub store: Option<stripe_shared::PaymentMethodDetailsKonbiniStore>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsKonbiniBuilder {
    store: Option<Option<stripe_shared::PaymentMethodDetailsKonbiniStore>>,
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

    impl Deserialize for PaymentMethodDetailsKonbini {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsKonbini>,
        builder: PaymentMethodDetailsKonbiniBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsKonbini> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsKonbiniBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsKonbiniBuilder {
        type Out = PaymentMethodDetailsKonbini;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "store" => Deserialize::begin(&mut self.store),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { store: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(store),) = (self.store.take(),) else {
                return None;
            };
            Some(Self::Out { store })
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

    impl ObjectDeser for PaymentMethodDetailsKonbini {
        type Builder = PaymentMethodDetailsKonbiniBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsKonbini {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsKonbiniBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "store" => b.store = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
