#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction {
    /// The [Refund](https://docs.stripe.com/api/refunds/object) that moved these funds into the customer's cash balance.
    pub refund: stripe_types::Expandable<stripe_shared::Refund>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder
{
    refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
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
            builder: CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder { refund: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "refund" => Deserialize::begin(&mut self.builder.refund),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(refund),) = (self.builder.refund.take(),) else {
                return Ok(());
            };
            *self.out = Some(CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction { refund });
            Ok(())
        }
    }
};
