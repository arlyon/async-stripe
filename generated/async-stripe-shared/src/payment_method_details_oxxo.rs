#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsOxxo {
    /// OXXO reference number
    pub number: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsOxxoBuilder {
    number: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsOxxo {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsOxxo>,
        builder: PaymentMethodDetailsOxxoBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsOxxo> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsOxxoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsOxxoBuilder {
        type Out = PaymentMethodDetailsOxxo;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "number" => Deserialize::begin(&mut self.number),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(number),) = (self.number.take(),) else {
                return None;
            };
            Some(Self::Out { number })
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

    impl ObjectDeser for PaymentMethodDetailsOxxo {
        type Builder = PaymentMethodDetailsOxxoBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsOxxo {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsOxxoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "number" => b.number = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
