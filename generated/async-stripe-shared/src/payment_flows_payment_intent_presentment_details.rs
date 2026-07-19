#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPaymentIntentPresentmentDetails {
    /// Amount intended to be collected by this payment, denominated in `presentment_currency`.
    pub presentment_amount: i64,
    /// Currency presented to the customer during payment.
    pub presentment_currency: stripe_types::Currency,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPaymentIntentPresentmentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPaymentIntentPresentmentDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPaymentIntentPresentmentDetailsBuilder {
    presentment_amount: Option<i64>,
    presentment_currency: Option<stripe_types::Currency>,
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

    impl Deserialize for PaymentFlowsPaymentIntentPresentmentDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPaymentIntentPresentmentDetails>,
        builder: PaymentFlowsPaymentIntentPresentmentDetailsBuilder,
    }

    impl Visitor for Place<PaymentFlowsPaymentIntentPresentmentDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsPaymentIntentPresentmentDetailsBuilder {
                    presentment_amount: Deserialize::default(),
                    presentment_currency: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "presentment_amount" => Deserialize::begin(&mut self.builder.presentment_amount),
                "presentment_currency" => {
                    Deserialize::begin(&mut self.builder.presentment_currency)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(presentment_amount), Some(presentment_currency)) =
                (self.builder.presentment_amount, self.builder.presentment_currency.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsPaymentIntentPresentmentDetails {
                presentment_amount,
                presentment_currency,
            });
            Ok(())
        }
    }
};
