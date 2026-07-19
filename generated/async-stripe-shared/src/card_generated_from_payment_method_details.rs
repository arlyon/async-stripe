#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CardGeneratedFromPaymentMethodDetails {
    pub card_present: Option<stripe_shared::PaymentMethodDetailsCardPresent>,
    /// The type of payment method transaction-specific details from the transaction that generated this `card` payment method.
    /// Always `card_present`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CardGeneratedFromPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CardGeneratedFromPaymentMethodDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CardGeneratedFromPaymentMethodDetailsBuilder {
    card_present: Option<Option<stripe_shared::PaymentMethodDetailsCardPresent>>,
    type_: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: CardGeneratedFromPaymentMethodDetailsBuilder {
                    card_present: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_present" => Deserialize::begin(&mut self.builder.card_present),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(card_present), Some(type_)) =
                (self.builder.card_present.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(CardGeneratedFromPaymentMethodDetails { card_present, type_ });
            Ok(())
        }
    }
};
