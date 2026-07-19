#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {
    /// The [Payment Intent](https://docs.stripe.com/api/payment_intents/object) that funds were applied to.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder {
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
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

    impl Deserialize
        for CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction,
        >,
        builder:
            CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder,
    }

    impl Visitor
        for Place<CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder { payment_intent: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_intent" => Deserialize::begin(&mut self.builder.payment_intent),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payment_intent),) = (self.builder.payment_intent.take(),) else {
                return Ok(());
            };
            *self.out = Some(
                CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {
                    payment_intent,
                },
            );
            Ok(())
        }
    }
};
