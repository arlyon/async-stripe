#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodCardGeneratedCard {
    /// The charge that created this object.
    pub charge: Option<String>,
    /// Transaction-specific details of the payment method used in the payment.
    pub payment_method_details: Option<stripe_shared::CardGeneratedFromPaymentMethodDetails>,
    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodCardGeneratedCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodCardGeneratedCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodCardGeneratedCardBuilder {
    charge: Option<Option<String>>,
    payment_method_details: Option<Option<stripe_shared::CardGeneratedFromPaymentMethodDetails>>,
    setup_attempt: Option<Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>>,
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

    impl Deserialize for PaymentMethodCardGeneratedCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCardGeneratedCard>,
        builder: PaymentMethodCardGeneratedCardBuilder,
    }

    impl Visitor for Place<PaymentMethodCardGeneratedCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodCardGeneratedCardBuilder {
                    charge: Deserialize::default(),
                    payment_method_details: Deserialize::default(),
                    setup_attempt: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.builder.charge),
                "payment_method_details" => {
                    Deserialize::begin(&mut self.builder.payment_method_details)
                }
                "setup_attempt" => Deserialize::begin(&mut self.builder.setup_attempt),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(charge), Some(payment_method_details), Some(setup_attempt)) = (
                self.builder.charge.take(),
                self.builder.payment_method_details.take(),
                self.builder.setup_attempt.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodCardGeneratedCard {
                charge,
                payment_method_details,
                setup_attempt,
            });
            Ok(())
        }
    }
};
