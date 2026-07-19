#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPaymentDetails {
    /// A unique value to identify the customer. This field is available only for card payments.
    ///
    /// This field is truncated to 25 alphanumeric characters, excluding spaces, before being sent to card networks.
    pub customer_reference: Option<String>,
    /// A unique value assigned by the business to identify the transaction. Required for L2 and L3 rates.
    ///
    /// For Cards, this field is truncated to 25 alphanumeric characters, excluding spaces, before being sent to card networks.
    /// For Klarna, this field is truncated to 255 characters and is visible to customers when they view the order in the Klarna app.
    pub order_reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPaymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPaymentDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPaymentDetailsBuilder {
    customer_reference: Option<Option<String>>,
    order_reference: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsPaymentDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPaymentDetails>,
        builder: PaymentFlowsPaymentDetailsBuilder,
    }

    impl Visitor for Place<PaymentFlowsPaymentDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsPaymentDetailsBuilder {
                    customer_reference: Deserialize::default(),
                    order_reference: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_reference" => Deserialize::begin(&mut self.builder.customer_reference),
                "order_reference" => Deserialize::begin(&mut self.builder.order_reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(customer_reference), Some(order_reference)) =
                (self.builder.customer_reference.take(), self.builder.order_reference.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsPaymentDetails { customer_reference, order_reference });
            Ok(())
        }
    }
};
