#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction {
    /// The [Refund](https://stripe.com/docs/api/refunds/object) that moved these funds into the customer's cash balance.
    pub refund: stripe_types::Expandable<stripe_shared::Refund>,
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder
{
    refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
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
        for CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
    out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>,
    builder: CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder,
}

    impl Visitor
        for Place<
            CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction,
        >
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder {
    type Out = CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "refund" => Deserialize::begin(&mut self.refund),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            refund: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(refund),
) = (self.refund.take(),
) else {
            return None;
        };
        Some(Self::Out { refund })
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
        for CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction
    {
        type Builder = CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder;
    }

    impl FromValueOpt
        for CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "refund" => b.refund = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
