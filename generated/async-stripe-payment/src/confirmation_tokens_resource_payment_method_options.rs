/// Payment-method-specific configuration
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourcePaymentMethodOptions {
    /// This hash contains the card payment method options.
    pub card: Option<stripe_payment::ConfirmationTokensResourcePaymentMethodOptionsResourceCard>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokensResourcePaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmationTokensResourcePaymentMethodOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConfirmationTokensResourcePaymentMethodOptionsBuilder {
    card:
        Option<Option<stripe_payment::ConfirmationTokensResourcePaymentMethodOptionsResourceCard>>,
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
                builder: ConfirmationTokensResourcePaymentMethodOptionsBuilder {
                    card: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.builder.card),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(card),) = (self.builder.card.take(),) else {
                return Ok(());
            };
            *self.out = Some(ConfirmationTokensResourcePaymentMethodOptions { card });
            Ok(())
        }
    }
};
