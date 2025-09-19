#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {
    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were applied to.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder {
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
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
            builder: CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder
    {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payment_intent: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payment_intent),) = (self.payment_intent.take(),) else {
                return None;
            };
            Some(Self::Out { payment_intent })
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

    impl ObjectDeser
        for CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction
    {
        type Builder =
            CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder;
    }

    impl FromValueOpt
        for CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
