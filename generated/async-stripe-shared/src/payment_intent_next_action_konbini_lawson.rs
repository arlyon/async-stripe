#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionKonbiniLawson {
    /// The confirmation number.
    pub confirmation_number: Option<String>,
    /// The payment code.
    pub payment_code: String,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionKonbiniLawsonBuilder {
    confirmation_number: Option<Option<String>>,
    payment_code: Option<String>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionKonbiniLawson {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionKonbiniLawson>,
        builder: PaymentIntentNextActionKonbiniLawsonBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionKonbiniLawson> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionKonbiniLawsonBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionKonbiniLawsonBuilder {
        type Out = PaymentIntentNextActionKonbiniLawson;
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
            let (Some(confirmation_number), Some(payment_code)) =
                (self.confirmation_number.take(), self.payment_code.take())
            else {
                return None;
            };
            Some(Self::Out { confirmation_number, payment_code })
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

    impl ObjectDeser for PaymentIntentNextActionKonbiniLawson {
        type Builder = PaymentIntentNextActionKonbiniLawsonBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionKonbiniLawson {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionKonbiniLawsonBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "confirmation_number" => b.confirmation_number = FromValueOpt::from_value(v),
                    "payment_code" => b.payment_code = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
