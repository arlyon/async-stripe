#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionKonbiniSeicomart {
    /// The confirmation number.
    pub confirmation_number: Option<String>,
    /// The payment code.
    pub payment_code: String,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionKonbiniSeicomartBuilder {
    confirmation_number: Option<Option<String>>,
    payment_code: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionKonbiniSeicomart {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionKonbiniSeicomart>,
        builder: PaymentIntentNextActionKonbiniSeicomartBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionKonbiniSeicomart> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionKonbiniSeicomartBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionKonbiniSeicomartBuilder {
        type Out = PaymentIntentNextActionKonbiniSeicomart;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "confirmation_number" => Deserialize::begin(&mut self.confirmation_number),
                "payment_code" => Deserialize::begin(&mut self.payment_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                confirmation_number: Deserialize::default(),
                payment_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                confirmation_number: self.confirmation_number.take()?,
                payment_code: self.payment_code.take()?,
            })
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

    impl ObjectDeser for PaymentIntentNextActionKonbiniSeicomart {
        type Builder = PaymentIntentNextActionKonbiniSeicomartBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionKonbiniSeicomart {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionKonbiniSeicomartBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "confirmation_number" => {
                        b.confirmation_number = Some(FromValueOpt::from_value(v)?)
                    }
                    "payment_code" => b.payment_code = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
