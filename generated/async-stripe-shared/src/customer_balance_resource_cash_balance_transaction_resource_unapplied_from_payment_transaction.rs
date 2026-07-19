#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction {
    /// The [Payment Intent](https://docs.stripe.com/api/payment_intents/object) that funds were unapplied from.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransactionBuilder
{
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
        for CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
    out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction>,
    builder: CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransactionBuilder,
}

    impl Visitor
        for Place<
            CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction,
        >
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransactionBuilder { payment_intent: Deserialize::default(),
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
            *self.out = Some(CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction { payment_intent });
            Ok(())
        }
    }
};
