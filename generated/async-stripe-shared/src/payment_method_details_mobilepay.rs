#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsMobilepay {
    /// Internal card details
    pub card: Option<stripe_shared::InternalCard>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsMobilepayBuilder {
    card: Option<Option<stripe_shared::InternalCard>>,
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

    impl Deserialize for PaymentMethodDetailsMobilepay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsMobilepay>,
        builder: PaymentMethodDetailsMobilepayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsMobilepay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsMobilepayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsMobilepayBuilder {
        type Out = PaymentMethodDetailsMobilepay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.card),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card),) = (self.card.take(),) else {
                return None;
            };
            Some(Self::Out { card })
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

    impl ObjectDeser for PaymentMethodDetailsMobilepay {
        type Builder = PaymentMethodDetailsMobilepayBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsMobilepay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsMobilepayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card" => b.card = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
