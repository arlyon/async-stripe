#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CardGeneratedFromPaymentMethodDetails {
    pub card_present: Option<stripe_shared::PaymentMethodDetailsCardPresent>,
    /// The type of payment method transaction-specific details from the transaction that generated this `card` payment method.
    /// Always `card_present`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
}
#[doc(hidden)]
pub struct CardGeneratedFromPaymentMethodDetailsBuilder {
    card_present: Option<Option<stripe_shared::PaymentMethodDetailsCardPresent>>,
    type_: Option<String>,
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

    impl Deserialize for CardGeneratedFromPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CardGeneratedFromPaymentMethodDetails>,
        builder: CardGeneratedFromPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<CardGeneratedFromPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CardGeneratedFromPaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CardGeneratedFromPaymentMethodDetailsBuilder {
        type Out = CardGeneratedFromPaymentMethodDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_present" => Deserialize::begin(&mut self.card_present),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card_present: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card_present), Some(type_)) = (self.card_present.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { card_present, type_ })
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

    impl ObjectDeser for CardGeneratedFromPaymentMethodDetails {
        type Builder = CardGeneratedFromPaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for CardGeneratedFromPaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CardGeneratedFromPaymentMethodDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_present" => b.card_present = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
