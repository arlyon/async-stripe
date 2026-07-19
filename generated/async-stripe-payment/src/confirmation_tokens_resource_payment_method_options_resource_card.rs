/// This hash contains the card payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourcePaymentMethodOptionsResourceCard {
    /// The `cvc_update` Token collected from the Payment Element.
pub cvc_token: Option<String>,
pub installments: Option<stripe_payment::ConfirmationTokensResourcePaymentMethodOptionsResourceCardResourceInstallment>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokensResourcePaymentMethodOptionsResourceCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmationTokensResourcePaymentMethodOptionsResourceCard")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConfirmationTokensResourcePaymentMethodOptionsResourceCardBuilder {
    cvc_token: Option<Option<String>>,
installments: Option<Option<stripe_payment::ConfirmationTokensResourcePaymentMethodOptionsResourceCardResourceInstallment>>,

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

    impl Deserialize for ConfirmationTokensResourcePaymentMethodOptionsResourceCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConfirmationTokensResourcePaymentMethodOptionsResourceCard>,
        builder: ConfirmationTokensResourcePaymentMethodOptionsResourceCardBuilder,
    }

    impl Visitor for Place<ConfirmationTokensResourcePaymentMethodOptionsResourceCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConfirmationTokensResourcePaymentMethodOptionsResourceCardBuilder {
                    cvc_token: Deserialize::default(),
                    installments: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cvc_token" => Deserialize::begin(&mut self.builder.cvc_token),
                "installments" => Deserialize::begin(&mut self.builder.installments),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(cvc_token), Some(installments)) =
                (self.builder.cvc_token.take(), self.builder.installments.take())
            else {
                return Ok(());
            };
            *self.out = Some(ConfirmationTokensResourcePaymentMethodOptionsResourceCard {
                cvc_token,
                installments,
            });
            Ok(())
        }
    }
};
