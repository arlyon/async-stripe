/// Payment-method-specific configuration
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourcePaymentMethodOptions {
    /// This hash contains the card payment method options.
    pub card: Option<stripe_payment::ConfirmationTokensResourcePaymentMethodOptionsResourceCard>,
}
#[doc(hidden)]
pub struct ConfirmationTokensResourcePaymentMethodOptionsBuilder {
    card:
        Option<Option<stripe_payment::ConfirmationTokensResourcePaymentMethodOptionsResourceCard>>,
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

    impl Deserialize for ConfirmationTokensResourcePaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConfirmationTokensResourcePaymentMethodOptions>,
        builder: ConfirmationTokensResourcePaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<ConfirmationTokensResourcePaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConfirmationTokensResourcePaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConfirmationTokensResourcePaymentMethodOptionsBuilder {
        type Out = ConfirmationTokensResourcePaymentMethodOptions;
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

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for ConfirmationTokensResourcePaymentMethodOptions {
        type Builder = ConfirmationTokensResourcePaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for ConfirmationTokensResourcePaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConfirmationTokensResourcePaymentMethodOptionsBuilder::deser_default();
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
